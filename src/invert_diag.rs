use faer::sparse::*;

pub fn invert_diag(matrix: &SparseColMat<usize, f32>) -> SparseColMat<usize, f32> {
  let mut triplets: Vec<Triplet<usize, usize, f32>> = vec![];
  for i in 0..matrix.ncols() {
    if let Some(val) = matrix.get(i, i) {
      if *val != 0.0 {
        triplets.push(Triplet::new(i, i, 1.0 / *val));
      }
    }
  }
  SparseColMat::<usize, f32>::try_new_from_triplets(matrix.nrows(), matrix.ncols(), &triplets)
    .unwrap()
}
