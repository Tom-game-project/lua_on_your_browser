use piccolo::{Lua, Closure,Callback, CallbackReturn, StaticError, Value, Table, Executor};
use piccolo::lua::*;
use std::io::Cursor;

use chrono::prelude::Utc;


pub fn set_plugin_standard(ctx:Context<'_>) -> Table<'_>{
    let table = Table::new(&ctx);

    let callback = Callback::from_fn(
        &ctx,
        |_, _, mut stack|{
            gloo::console::log!("hello this is test");
            // let a = CallbackReturn::Return;
            // Ok(Value::Boolean(true))
            stack.clear();
            let utc_now = Utc::now();
            stack.push_back(
                Value::Integer(
                    utc_now.timestamp() // utc time
                )
            );
            Ok(CallbackReturn::Return)
        }
    );
    let _ = table.set(ctx, "time", callback);
    table
}

pub fn lua_runtime(code:String)-> Result<(i32, i32, i32), StaticError>{
    let cursor = Cursor::new(code);
    let mut lua = Lua::core();
    lua.try_enter(|ctx| {
        let print_console = Callback::from_fn(&ctx, |_, _, mut stack| {
            let mut print_string:Vec<String>= Vec::new();
            for i in &stack{
                match i{
                    Value::String(s) => print_string.push(s.to_string()),
                    Value::Number(n) => print_string.push(format!("{}", n)),
                    Value::Integer(n) => print_string.push(format!("{}", n)),
                    Value::Boolean(b) => print_string.push(format!("{}", b)),
                    Value::Nil => print_string.push("nil".to_string()),
                    _ => gloo::console::log!("wrong type!")
                };
            }
            gloo::console::log!(
                print_string.join(" ")
            );
            Ok(CallbackReturn::Return)
        });
        let _ = ctx.set_global("print", print_console);
        // let _ = ctx.set_global("pi", Value::Number(3.14));
        let _ = ctx.set_global("os", Value::Table(set_plugin_standard(ctx)));
        Ok(())
    })?;
    let executor = lua.try_enter(|ctx| {
        let closure = Closure::load(
            ctx,
            None,
            cursor,
        )?;
        Ok(ctx.stash(Executor::start(ctx, closure.into(), ())))
    })?;
    let (a,b,c) = lua.execute::<(i32, i32, i32)>(&executor)?;
    Ok((a, b, c))
}

