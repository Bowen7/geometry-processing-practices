use faer::sparse::*;
use web_sys::console;
use web_time::Instant;

pub fn slice(
  matrix: &SparseColMat<usize, f32>,
  row_indices: &[usize],
  col_indices: &[usize],
) -> SparseColMat<usize, f32> {
  let nrows = matrix.nrows();
  let ncols = matrix.ncols();
  let row_size = row_indices.len();
  let col_size = col_indices.len();

  let mut triplets: Vec<Triplet<usize, usize, f32>> = vec![];
  // FIXME: consider using smallvec
  let mut row_reindex: Vec<Vec<usize>> = vec![vec![]; nrows];
  for i in 0..row_size {
    row_reindex[row_indices[i]].push(i);
  }

  let mut col_reindex: Vec<Vec<usize>> = vec![vec![]; ncols];
  for i in 0..col_size {
    col_reindex[col_indices[i]].push(i);
  }

  for j in 0..ncols {
    if col_reindex[j].is_empty() {
      continue;
    }
    matrix.row_idx_of_col(j).for_each(|i: usize| {
      if let Some(value) = matrix.get(i, j) {
        row_reindex[i].iter().for_each(|row_idx| {
          col_reindex[j].iter().for_each(|col_idx| {
            triplets.push(Triplet::new(*row_idx, *col_idx, *value));
          });
        });
      }
    });
  }

  SparseColMat::<usize, f32>::try_new_from_triplets(row_indices.len(), col_indices.len(), &triplets)
    .unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_slice() {
    let m = SparseColMat::<usize, f32>::try_new_from_triplets(
      4,
      4,
      &[
        Triplet::new(0, 0, 0.75),
        Triplet::new(0, 1, 0.2),
        Triplet::new(1, 0, 0.3),
        Triplet::new(1, 1, -2.0),
      ],
    )
    .unwrap();
    let submatrix = slice(&m, &[0], &[0, 1]);
    let dense = submatrix.to_dense();

    let expected = SparseColMat::<usize, f32>::try_new_from_triplets(
      1,
      2,
      &[Triplet::new(0, 0, 0.75), Triplet::new(0, 1, 0.2)],
    )
    .unwrap();
    assert_eq!(dense, expected.to_dense());
  }
}
