use faer::prelude::*;
use wasm_bindgen::prelude::*;

use crate::edges::*;
use crate::utils::*;

#[wasm_bindgen]
pub fn wasm_euler_characteristic(
  vertices: Vec<f32>,
  faces: Vec<usize>,
) -> Result<JsValue, JsError> {
  let F = build_F(faces);
  let V = build_V(vertices);
  let Chi = euler_characteristic(&V, &F);

  let result = js_sys::Object::new();
  js_sys::Reflect::set(&result, &"chi".into(), &Chi.into()).unwrap();

  Ok(result.into())
}

pub fn euler_characteristic(V: &Mat<f32>, F: &Mat<usize>) -> i64 {
  let E = edges(F, V.nrows());
  V.nrows() as i64 - E.nrows() as i64 + F.nrows() as i64
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_euler_characteristic() {
    let V = mat![
      [0.0, 0.0, 0.0],
      [1.0, 0.0, 0.0],
      [0.0, 1.0, 0.0],
      [0.0, 0.0, 1.0]
    ];
    let F = mat![[0, 1, 2], [2, 3, 0]];
    let Chi = euler_characteristic(&V, &F);
    assert_eq!(Chi, 1);
  }
}
