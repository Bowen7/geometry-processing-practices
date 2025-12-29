use faer::prelude::*;
use faer::sparse::*;
use wasm_bindgen::prelude::*;

use crate::adjacency_matrix::*;
use crate::utils::*;

#[wasm_bindgen]
pub fn wasm_edges(faces: Vec<usize>, vertex_count: usize) -> Result<JsValue, JsError> {
  let F = build_faces(faces);
  let E = edges(&F, vertex_count);
  let mut edges = Vec::with_capacity(E.nrows() * E.ncols());
  for i in 0..E.nrows() {
    for j in 0..E.ncols() {
      edges.push(E[(i, j)]);
    }
  }

  let result = js_sys::Object::new();
  js_sys::Reflect::set(&result, &"edges".into(), &edges.into()).unwrap();

  Ok(result.into())
}

pub fn edges(F: &Mat<usize>, vertex_count: usize) -> Mat<usize> {
  let A = adjacency_matrix(&F, vertex_count);
  let edge_count = A.val().len() / 2;
  let mut E = Mat::<usize>::full(edge_count, 2, 0);
  let mut i = 0;
  A.triplet_iter().for_each(|Triplet { row, col, val: _ }| {
    if row < col {
      E[(i, 0)] = row;
      E[(i, 1)] = col;
      i += 1;
    }
  });
  E
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_edges() {
    let F = mat![[0, 1, 2], [2, 3, 0]];
    let vertex_count = 4;
    let edges = edges(&F, vertex_count);
    assert_eq!(edges, mat![[0, 1], [0, 2], [1, 2], [0, 3], [2, 3]]);
  }
}
