use crate::doublearea::*;
use crate::edge_lengths::*;
use crate::squared_edge_lengths::*;
use faer::sparse::*;
use faer::Mat;

pub fn cotmatrix(V: &Mat<f32>, F: &Mat<usize>) -> SparseColMat<usize, f32> {
  let vertex_count = V.nrows();
  let face_count = F.nrows();
  let l2 = squared_edge_lengths(V, F);
  let l = edge_lengths(&l2);
  let double_areas = doublearea(&l);

  let mut cotangent = Mat::<f32>::zeros(face_count, 3);
  for i in 0..face_count {
    cotangent[(i, 0)] = (l2[(i, 1)] + l2[(i, 2)] - l2[(i, 0)]) / (4.0 * double_areas[i]);
    cotangent[(i, 1)] = (l2[(i, 0)] + l2[(i, 2)] - l2[(i, 1)]) / (4.0 * double_areas[i]);
    cotangent[(i, 2)] = (l2[(i, 0)] + l2[(i, 1)] - l2[(i, 2)]) / (4.0 * double_areas[i]);
  }

  let mut triplets: Vec<Triplet<usize, usize, f32>> = vec![];
  for i in 0..face_count {
    let origin = F.row(i)[0];
    let dest = F.row(i)[1];
    let apex = F.row(i)[2];

    let cot = cotangent[(i, 0)];
    triplets.push(Triplet::new(dest, apex, cot));
    triplets.push(Triplet::new(apex, dest, cot));
    triplets.push(Triplet::new(dest, dest, -cot));
    triplets.push(Triplet::new(apex, apex, -cot));

    let cot = cotangent[(i, 1)];
    triplets.push(Triplet::new(origin, apex, cot));
    triplets.push(Triplet::new(apex, origin, cot));
    triplets.push(Triplet::new(origin, origin, -cot));
    triplets.push(Triplet::new(apex, apex, -cot));

    let cot = cotangent[(i, 2)];
    triplets.push(Triplet::new(dest, origin, cot));
    triplets.push(Triplet::new(origin, dest, cot));
    triplets.push(Triplet::new(dest, dest, -cot));
    triplets.push(Triplet::new(origin, origin, -cot));
  }

  let mat =
    SparseColMat::<usize, f32>::try_new_from_triplets(vertex_count, vertex_count, &triplets)
      .unwrap();
  mat
}

#[cfg(test)]
mod tests {
  use super::*;
  use faer::mat;

  #[test]
  fn test_cot_matrix() {
    let v = mat![[0.0, 4.0], [-3.0, 0.0], [0.0, 0.0], [3.0, 0.0]];
    let f = mat![[0, 1, 2], [0, 2, 3]];
    let c = cotmatrix(&v, &f);
    let dense = c.to_dense();
    assert_eq!(
      dense,
      mat![
        [-0.75, 0., 0.75, 0.],
        [0., -2.0 / 3.0, 2.0 / 3.0, 0.],
        [0.75, 2.0 / 3.0, -2.083333333333333, 2.0 / 3.0],
        [0., 0., 2.0 / 3.0, -2.0 / 3.0]
      ]
    );
  }
}
