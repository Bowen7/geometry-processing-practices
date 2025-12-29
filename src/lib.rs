#![allow(non_snake_case)]

use wasm_bindgen::prelude::*;

mod adjacency_matrix;
mod edges;
mod euler_characteristic;
mod utils;

#[wasm_bindgen]
pub fn cube() -> Result<JsValue, JsError> {
  let result = js_sys::Object::new();
  let args = vec![1.0, 1.0, 1.0];
  js_sys::Reflect::set(&result, &"args".into(), &args.into()).unwrap();

  Ok(result.into())
}
