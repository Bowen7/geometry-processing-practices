use faer::prelude::*;
use faer::sparse::*;

pub fn adjacency_matrix(faces: &Mat<usize>, vertex_count: usize) -> SparseColMat<usize, f64> {
  let m = faces.nrows();
  let mut triplets: Vec<Triplet<usize, usize, f64>> = vec![];
  for i in 0..m {
    for j in 0..3 {
      let row = *faces.get(i, j);
      let col = *faces.get(i, (j + 1) % 3);
      triplets.push(Triplet::new(row, col, 1.0));
      triplets.push(Triplet::new(col, row, 1.0));
    }
  }
  SparseColMat::<usize, f64>::try_new_from_triplets(vertex_count, vertex_count, &triplets).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_adjacency_matrix() {
    let A = mat![[1, 2, 3]];
    let vertex_count = 4;
    let A = adjacency_matrix(&A, vertex_count);
    assert_eq!(
      A.to_dense(),
      mat![
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 1.0],
        [0.0, 1.0, 0.0, 1.0],
        [0.0, 1.0, 1.0, 0.0],
      ]
    );
  }
}
