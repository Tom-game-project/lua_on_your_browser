use std::sync::{Arc, Mutex};
// piccolo
use piccolo::{Lua, Closure,Callback, CallbackReturn, StaticError, Value, Table, Executor};
use piccolo::lua::*;
use std::io::Cursor;

use chrono::prelude::Utc;


pub fn set_plugin_standard(ctx:Context<'_>) -> Table<'_>{
    let table = Table::new(&ctx);

    let callback = Callback::from_fn(
        &ctx,
        |_, _, mut stack|{
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


pub fn set_plugin_console<'a>(ctx: Context<'a>, stdout: Arc<Mutex<String>>) -> Table<'a> {
    let table = Table::new(&ctx);

    let callback_stdout = Arc::clone(&stdout); // クロージャ内で使用するためにcloneする

    let callback = Callback::from_fn(
        &ctx,
        move |_, _, mut stack| {
            stack.clear();
            // stdoutをロックして変更する
            let mut stdout_locked = callback_stdout.lock().unwrap();
            *stdout_locked = String::from("hello world");
            Ok(CallbackReturn::Return)
        },
    );
    
    let _ = table.set(ctx, "print_hello", callback);
    table
}


pub fn lua_runtime(code:String,stdout:Arc<Mutex<String>>)-> Result<(i32, i32, i32), StaticError>{
    let cursor = Cursor::new(code);
    let mut lua = Lua::core();
    
    let callback_stdout = Arc::clone(&stdout); // クロージャ内で使用するためにcloneする
    lua.try_enter(move |ctx| {
        let print_console = Callback::from_fn(&ctx, move |_, _, stack| {
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
            // gloo::console::log!(
            //     print_string.join(" ")
            // );
            let mut stdout_locked = callback_stdout.lock().unwrap();
            *stdout_locked = format!("{}{}\n",*stdout_locked, print_string.join(" "));
            Ok(CallbackReturn::Return)
        });

        let _ = ctx.set_global("print", print_console);
        let _ = ctx.set_global("pi", Value::Number(3.14));
        let _ = ctx.set_global("os", Value::Table(set_plugin_standard(ctx)));
        let _ = ctx.set_global("console", Value::Table(set_plugin_console(
            ctx,
            stdout
        )));
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

