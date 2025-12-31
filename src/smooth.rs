use crate::cotmatrix::*;
use crate::massmatrix::*;
use crate::utils::*;
use faer::prelude::Solve;
use faer::Mat;
use faer::Side;
use wasm_bindgen::prelude::*;

pub fn smooth(V: &Mat<f32>, F: &Mat<usize>, lambda: f64, n: usize) -> Mat<f32> {
  if n == 0 {
    return V.clone();
  }

  let L = cotmatrix(V, F);
  let M = massmatrix(V, F, None);
  let A = &M - lambda * &L;
  let llt = A.sp_cholesky(Side::Lower).unwrap();

  let mut U = V.clone();
  for _ in 0..n {
    let rhs = &M * &U;
    U = llt.solve(&rhs);
  }

  U
}

#[wasm_bindgen]
pub fn wasm_smooth(
  vertices: Vec<f32>,
  faces: Vec<usize>,
  lambda: f64,
  n: usize,
) -> Result<JsValue, JsError> {
  let V = build_V(vertices);
  let F = build_F(faces);
  let U = smooth(&V, &F, lambda, n);

  let result = js_sys::Object::new();
  js_sys::Reflect::set(&result, &"vertices".into(), &build_vertices(U).into()).unwrap();

  Ok(result.into())
}
