use crate::doublearea::*;
use crate::edge_lengths::*;
use crate::squared_edge_lengths::*;
use faer::sparse::*;
use faer::Mat;

pub fn massmatrix(
  V: &Mat<f32>,
  F: &Mat<usize>,
  areas: Option<&Vec<f32>>,
) -> SparseColMat<usize, f32> {
  let vertex_count = V.nrows();
  let mut triplets: Vec<Triplet<usize, usize, f32>> = vec![];

  let computed_areas;
  let areas_slice: &[f32] = if let Some(areas) = areas {
    areas
  } else {
    let l2 = squared_edge_lengths(V, F);
    let l = edge_lengths(&l2);
    let double_areas = doublearea(&l);
    computed_areas = double_areas.iter().map(|x| x / 2.0).collect::<Vec<f32>>();
    &computed_areas
  };

  let nrows = F.nrows();
  for row_idx in 0..nrows {
    let v1 = F.row(row_idx)[0];
    let v2 = F.row(row_idx)[1];
    let v3 = F.row(row_idx)[2];
    let area = areas_slice[row_idx];
    triplets.push(Triplet::new(v1, v1, area));
    triplets.push(Triplet::new(v2, v2, area));
    triplets.push(Triplet::new(v3, v3, area));
  }

  let mat =
    SparseColMat::<usize, f32>::try_new_from_triplets(vertex_count, vertex_count, &triplets)
      .unwrap();
  mat / 3.0
}

#[cfg(test)]
mod tests {
  use super::*;
  use faer::mat;

  #[test]
  fn test_massmatrix() {
    let V = mat![
      [0.0, 0.0, 0.0],
      [0.0, 4.0, 0.0],
      [-3.0, 0.0, 0.0],
      [0.0, -4.0, 0.0]
    ];
    let F = mat![[0, 1, 2], [0, 2, 3]];
    // let a = vec![6.0, 6.0];
    let m = massmatrix(&V, &F, None);
    let expected: csc_numeric::generic::SparseColMat<csc_numeric::Own<usize, f32>> =
      SparseColMat::<usize, f32>::try_new_from_triplets(
        4,
        4,
        &[
          Triplet::new(0, 0, 4.0),
          Triplet::new(1, 1, 2.0),
          Triplet::new(2, 2, 4.0),
          Triplet::new(3, 3, 2.0),
        ],
      )
      .unwrap();
    assert_eq!(m.to_dense(), expected.to_dense());
  }
}
