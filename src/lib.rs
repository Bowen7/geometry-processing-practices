use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn cube() -> Result<JsValue, JsError> {
  let result = js_sys::Object::new();
  let args = vec![1.0, 1.0, 1.0];
  js_sys::Reflect::set(&result, &"args".into(), &args.into()).unwrap();

  Ok(result.into())
}
