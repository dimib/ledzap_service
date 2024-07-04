
use wasm_bindgen::JsValue;
use gloo_console::log;

pub fn log(s: &String) {
    let obj = JsValue::from(s);
    log!("text", obj);
}
