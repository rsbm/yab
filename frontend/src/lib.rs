use wasm_bindgen::prelude::*;

mod components;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<components::api::Model>();
    Ok(())
}
