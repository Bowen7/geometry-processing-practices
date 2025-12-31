use faer::Mat;

pub fn squared_edge_lengths(vertices: &Mat<f32>, faces: &Mat<usize>) -> Mat<f32> {
  let m = faces.nrows();
  let mut l2 = Mat::<f32>::zeros(m, 3);
  for i in 0..m {
    let t = faces.row(i);
    let v0 = vertices.row(t[0]);
    let v1 = vertices.row(t[1]);
    let v2 = vertices.row(t[2]);
    l2[(i, 0)] = (v1 - v2).squared_norm_l2();
    l2[(i, 1)] = (v0 - v2).squared_norm_l2();
    l2[(i, 2)] = (v0 - v1).squared_norm_l2();
  }
  l2
}

#[cfg(test)]
mod tests {
  use super::*;
  use faer::mat;

  #[test]
  fn test_squared_edge_lengths_1() {
    let v = mat![[1.0, 2.0], [2.0, 1.0], [1.0, 1.0]];
    let f = mat![[0, 1, 2]];
    let edge_lengths = squared_edge_lengths(&v, &f);
    assert_eq!(edge_lengths, mat![[1.0, 1.0, 2.0]]);
  }

  #[test]
  fn test_squared_edge_lengths_2() {
    let v = mat![[1.0, 2.0, 0.0], [2.0, 1.0, 0.0], [1.0, 1.0, 0.0]];
    let f = mat![[0, 1, 2]];
    let edge_lengths = squared_edge_lengths(&v, &f);
    assert_eq!(edge_lengths, mat![[1.0, 1.0, 2.0]]);
  }
}
