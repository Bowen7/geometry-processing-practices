use faer::Mat;

use crate::cotmatrix::*;
use crate::invert_diag::*;
use crate::massmatrix::*;
use crate::min_quad_with_fixed::*;
use crate::utils::*;
use wasm_bindgen::prelude::*;
use web_sys::console;
use web_time::Instant;

pub fn biharmonic_precompute(V: &Mat<f32>, F: &Mat<usize>, b: &Vec<usize>) -> MinQuadData {
  let M = massmatrix(V, F, None);
  let L = cotmatrix(V, F);
  let Q = L.transpose().to_col_major().unwrap() * (&(invert_diag(&M) * L));
  min_quad_with_fixed_precompute(&Q, b, false)
}

pub fn biharmonic_solve(data: &MinQuadData, known_values: &Mat<f32>) -> Option<Mat<f32>> {
  let B = Mat::zeros(data.n, 3);
  min_quad_with_fixed_solve(data, &B, known_values)
}

#[wasm_bindgen]
pub fn wasm_biharmonic_precompute(
  vertices: Vec<f32>,
  faces: Vec<usize>,
  b: Vec<usize>,
) -> Result<JsValue, JsError> {
  let V = build_V(vertices);
  let F = build_F(faces);
  let data = biharmonic_precompute(&V, &F, &b);
  Ok(data.into())
}

#[wasm_bindgen]
pub fn wasm_biharmonic_solve(
  data: &MinQuadData,
  known_values: Vec<f32>,
) -> Result<JsValue, JsError> {
  let known_values = build_V(known_values);
  let U = biharmonic_solve(data, &known_values);
  let result = js_sys::Object::new();
  if let Some(U) = U {
    js_sys::Reflect::set(&result, &"displacements".into(), &build_vertices(U).into()).unwrap();
  }
  Ok(result.into())
}
