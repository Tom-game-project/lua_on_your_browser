use wasm_bindgen::prelude::*;
use piccolo::{Lua, Callback, Closure, CallbackReturn, StaticError, Value, Executor};

fn func()-> Result<(i32, i32, i32), StaticError>{
        let mut lua = Lua::core();

    lua.try_enter(|ctx| {
        let callback = Callback::from_fn(&ctx, |_, _, mut stack| {
            stack.push_back(Value::Integer(42));
            Ok(CallbackReturn::Return)
        });
        ctx.set_global("callback", callback);
        Ok(())
    })?;

    let executor = lua.try_enter(|ctx| {
        let closure = Closure::load(
            ctx,
            None,
            &br#"
                local a, b, c = callback(1, 2)
                assert(a == 1 and b == 2 and c == 42)
                local d, e, f = callback(3, 4)
                assert(d == 3 and e == 4 and f == 42)
                return a,b,c
            "#[..],
        )?;

        Ok(ctx.stash(Executor::start(ctx, closure.into(), ())))
    })?;

    let (a,b,c) = lua.execute::<(i32, i32, i32)>(&executor)?;
    Ok((a, b, c))
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[wasm_bindgen]
pub fn wasm_func(){
    let (a,b,c) = func().unwrap();
    gloo::console::log!(format!("{},{},{}",a,b,c));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works1() {
        func();
    }
}
