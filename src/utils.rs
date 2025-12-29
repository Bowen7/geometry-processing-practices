use faer::prelude::*;

pub fn build_vertices(vertices: Vec<f64>) -> Mat<f64> {
  Mat::from_fn(vertices.len() / 3, 3, |i, j| vertices[i * 3 + j])
}

pub fn build_faces(faces: Vec<usize>) -> Mat<usize> {
  Mat::from_fn(faces.len() / 3, 3, |i, j| faces[i * 3 + j])
}
