use faer::prelude::*;

pub fn build_V(vertices: Vec<f32>) -> Mat<f32> {
  Mat::from_fn(vertices.len() / 3, 3, |i, j| vertices[i * 3 + j])
}

pub fn build_F(faces: Vec<usize>) -> Mat<usize> {
  Mat::from_fn(faces.len() / 3, 3, |i, j| faces[i * 3 + j])
}

pub fn build_vertices(V: Mat<f32>) -> Vec<f32> {
  let mut vertices = Vec::with_capacity(V.nrows() * V.ncols());
  for i in 0..V.nrows() {
    for j in 0..V.ncols() {
      vertices.push(V[(i, j)]);
    }
  }
  vertices
}

pub fn build_faces(F: Mat<usize>) -> Vec<usize> {
  let mut faces = Vec::with_capacity(F.nrows() * F.ncols());
  for i in 0..F.nrows() {
    for j in 0..F.ncols() {
      faces.push(F[(i, j)]);
    }
  }
  faces
}
