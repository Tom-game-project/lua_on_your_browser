use web_sys::window;

use wasm_bindgen::prelude::*;

use web_sys::HtmlElement;
use serde::Serialize;

use yew::prelude::*;
use yew::html::Scope;

use js_sys::Reflect;
use gloo_utils::format::JsValueSerdeExt;
use crate::lua_logic::lua_runtime::lua_runtime;

// CodeMirrorの初期化関数を呼び出す
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = CodeMirror)]
    fn fromTextArea(text_area: &HtmlElement, options: &JsValue) -> JsValue;
}

#[derive(Serialize)]
struct CodeMirrorData{
    // value: String,
    mode:  String,
    lineNumbers: bool,
}

pub enum PluginState{
    InitTextArea,
}

pub struct IndexComponent{
    link:Scope<Self>,
    codemirror:JsValue
}

impl Component for IndexComponent{
    type Message = PluginState;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            link: ctx.link().clone(),
            codemirror:JsValue::null()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PluginState::InitTextArea => {
                // pass
                let js_value = &self.codemirror;
                    let prototype = Reflect::get_prototype_of(&js_value).unwrap();
                let method = if let Ok(v) = Reflect::get(
                    &prototype,
                    &JsValue::from_str("getValue"),
                ){
                    v
                }else{
                    JsValue::from_str("error!")
                };
                if let Some(function) = method.dyn_into::<js_sys::Function>().ok() {
                    // JavaScript関数を呼び出す
                    let this = js_value.into();
                    if let Ok(v) = function.call0(&this){
                        // gloo::console::log!(v);
                        if let Ok(lua_result) = lua_runtime(v.as_string().expect("this is not string")){
                            let (l_a,l_b,l_c) = lua_result;
                            gloo::console::log!(format!("return {}, {}, {}", l_a, l_b, l_c));
                        } else {
                            gloo::console::log!("some error occured!");
                        }
                    } else {
                        gloo::console::log!("somethig wrong");
                    }
                } else {
                    gloo::console::log!("JsValue is not a function!");
                }
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let callback = self.link.callback(move |_:MouseEvent| PluginState::InitTextArea);
        html!(
            <>
                <h1 class="text-3xl font-semibold">{"Lua on your browser"}</h1>
                <div>
                    <button onclick={callback}>{"Run"}</button>
                </div>
                <div>
                    <textarea id={"lua_code_area"} value={r#"print(os.time())
return 1,2,3
"#}></textarea>
                </div>
            </>
        )
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        // 初回レンダリング後にコードを実行
        if first_render {
            self.link.send_message(PluginState::InitTextArea);
            let text_area_id = "lua_code_area";
            let window = window().unwrap();
            let document = window.document().unwrap();
            let text_area = document.get_element_by_id(text_area_id).unwrap();
            let text_area = text_area.dyn_into::<HtmlElement>().unwrap();
            let data = CodeMirrorData {
                mode: "lua".to_string(),
                lineNumbers:true
            };
            // CodeMirrorのオプションを設定
            let options = JsValue::from_serde(&data).unwrap();
            // CodeMirrorの初期化を呼び出す
            let js_value =fromTextArea(&text_area, &options);
            self.codemirror = js_value;
        }
    }
}