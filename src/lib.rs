use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use piccolo::{Lua, Callback, Closure, CallbackReturn, StaticError, Value, Executor};
// web-sys
use web_sys::HtmlElement;
use web_sys::window;
// gloo
use gloo_utils::format::JsValueSerdeExt;
// serde
use serde::Serialize;

use std::io::Cursor;
use js_sys;
use js_sys::Reflect;


#[derive(Serialize)]
struct CodeMirrorData{
    value: String,
    mode:  String,
    lineNumbers: bool,
}

// CodeMirrorの初期化関数を呼び出す
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = CodeMirror)]
    fn fromTextArea(text_area: &HtmlElement, options: &JsValue) -> JsValue;
}

fn func(code:String)-> Result<(i32, i32, i32), StaticError>{
    let cursor = Cursor::new(code);
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
            cursor,
        )?;

        Ok(ctx.stash(Executor::start(ctx, closure.into(), ())))
    })?;
    let (a,b,c) = lua.execute::<(i32, i32, i32)>(&executor)?;
    Ok((a, b, c))
}

#[wasm_bindgen]
pub fn set_highlight(){
    let text_area_id = "my_text_area";
    let window = window().unwrap();
    let document = window.document().unwrap();
    let text_area = document.get_element_by_id(text_area_id).unwrap();
    let text_area = text_area.dyn_into::<HtmlElement>().unwrap();

    let data = CodeMirrorData {
        value:r#"
    local a, b, c = callback(1, 2)
    assert(a == 1 and b == 2 and c == 42)
    local d, e, f = callback(3, 4)
    assert(d == 3 and e == 4 and f == 42)
    return a,b,c
        "#.to_string(),
        mode: "lua".to_string(),
        lineNumbers:true
    };
    // CodeMirrorのオプションを設定
    let options = JsValue::from_serde(&data).unwrap();
    // CodeMirrorの初期化を呼び出す
    let js_value =fromTextArea(&text_area, &options);

    gloo::console::log!(js_value.clone());
    // ここまでOK
    let prototype = Reflect::get_prototype_of(&js_value).unwrap();
    let method = if let Ok(v) = Reflect::get(
        &prototype,
        &JsValue::from_str("getValue"),
    ){
        gloo::console::log!("getValue success!");
        gloo::console::log!(v.clone());
        gloo::console::log!(prototype.clone());
        v
    }else{
        gloo::console::log!("getValue failure!");
        JsValue::from_str("error!")
    };
    if let Some(function) = method.dyn_into::<js_sys::Function>().ok() {
        // JavaScript関数を呼び出す
        gloo::console::log!("JsValue is a function!");
        let this = js_value.into();
        // let args = vec![
        //     JsValue::from_str("Hello"),
        //     JsValue::from_str("from Rust!"),
        // ];
        if let Ok(v) = function.call0(&this){
            gloo::console::log!(v);
        } else {
            gloo::console::log!("somethig wrong");
        }
    } else {
        gloo::console::log!("JsValue is not a function!");
        // console::log_1(&JsValue::from_str("JsValue is not a function!"));
    }
}


#[wasm_bindgen]
pub fn wasm_func(){
    let (a,b,c) = func(r#"
    local a, b, c = callback(1, 2)
    assert(a == 1 and b == 2 and c == 42)
    local d, e, f = callback(3, 4)
    assert(d == 3 and e == 4 and f == 42)
    return a,b,c
    "#.to_string()).unwrap();
    gloo::console::log!(format!("{},{},{}",a,b,c));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        func(r#"
    local a, b, c = callback(1, 2)
    assert(a == 1 and b == 2 and c == 42)
    local d, e, f = callback(3, 4)
    assert(d == 3 and e == 4 and f == 42)
    return a,b,c
    "#.to_string());
    }
}
