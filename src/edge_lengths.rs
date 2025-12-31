use faer::Mat;

pub fn edge_lengths(l2: &Mat<f32>) -> Mat<f32> {
  let mut l = Mat::<f32>::zeros(l2.nrows(), 3);
  for i in 0..l2.nrows() {
    l[(i, 0)] = l2[(i, 0)].sqrt();
    l[(i, 1)] = l2[(i, 1)].sqrt();
    l[(i, 2)] = l2[(i, 2)].sqrt();
  }
  l
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::squared_edge_lengths::*;
  use faer::mat;

  #[test]
  fn test_edge_lengths() {
    let V = mat![[1.0, 2.0, 0.0], [2.0, 1.0, 0.0], [1.0, 1.0, 0.0]];
    let F = mat![[0, 1, 2]];
    let l2 = squared_edge_lengths(&V, &F);
    let edge_lengths = edge_lengths(&l2);
    assert_eq!(edge_lengths, mat![[1.0, 1.0, 2.0_f32.sqrt()]]);
  }
}
