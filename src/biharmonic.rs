use faer::sparse::*;
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
pub struct BiharmonicHandler {
  data: MinQuadData,
}

#[wasm_bindgen]
impl BiharmonicHandler {
  #[wasm_bindgen(constructor)]
  pub fn new(vertices: Vec<f32>, faces: Vec<usize>, b: Vec<usize>) -> Self {
    let V = build_V(vertices);
    let F = build_F(faces);
    let data = biharmonic_precompute(&V, &F, &b);
    Self { data }
  }

  #[wasm_bindgen]
  pub fn solve(&self, known_values: Vec<f32>) -> Result<JsValue, JsError> {
    let known_values = build_V(known_values);
    let U = biharmonic_solve(&self.data, &known_values);
    let result = js_sys::Object::new();
    if let Some(U) = U {
      let displacements = build_vertices(U);
      js_sys::Reflect::set(&result, &"displacements".into(), &displacements.into()).unwrap();
    }
    Ok(result.into())
  }
}
