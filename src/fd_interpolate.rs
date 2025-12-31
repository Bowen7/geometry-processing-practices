use faer::prelude::*;
use faer::sparse::*;

/*
          A ----------- B
         /|            /|
        / |           / |
       /  |          /  |
      D ----------- C   |
      |   E --------|-- F
      |  /          |  /
      | /           | /
      |/            |/
      H ----------- G
*/
pub fn fd_interpolate(
  nx: usize,
  ny: usize,
  nz: usize,
  h: f32,
  corner: [f32; 3],
  P: Mat<f32>,
) -> SparseColMat<usize, f32> {
  let nrows = nx * ny * nz;
  let ncols = P.nrows();
  let mut triplets: Vec<Triplet<usize, usize, f32>> = vec![];

  let get_grid_index = |i: usize, j: usize, k: usize| i + nx * (j + ny * k);

  for index in 0..ncols {
    let x = (P[(index, 0)] - corner[0]) / h;
    let y = (P[(index, 1)] - corner[1]) / h;
    let z = (P[(index, 2)] - corner[2]) / h;

    let i = x.floor() as usize;
    let j = y.floor() as usize;
    let k = z.floor() as usize;

    let wx = x - i as f32;
    let wy = y - j as f32;
    let wz = z - k as f32;

    triplets.push(Triplet::new(
      get_grid_index(i, j, k),
      index,
      (1.0 - wx) * (1.0 - wy) * (1.0 - wz),
    ));
    triplets.push(Triplet::new(
      get_grid_index(i + 1, j, k),
      index,
      wx * (1.0 - wy) * (1.0 - wz),
    ));
    triplets.push(Triplet::new(
      get_grid_index(i, j + 1, k),
      index,
      (1.0 - wx) * wy * (1.0 - wz),
    ));
    triplets.push(Triplet::new(
      get_grid_index(i + 1, j + 1, k),
      index,
      wx * wy * (1.0 - wz),
    ));
    triplets.push(Triplet::new(
      get_grid_index(i, j, k + 1),
      index,
      (1.0 - wx) * (1.0 - wy) * wz,
    ));
    triplets.push(Triplet::new(
      get_grid_index(i + 1, j, k + 1),
      index,
      wx * (1.0 - wy) * wz,
    ));
    triplets.push(Triplet::new(
      get_grid_index(i, j + 1, k + 1),
      index,
      (1.0 - wx) * wy * wz,
    ));
    triplets.push(Triplet::new(
      get_grid_index(i + 1, j + 1, k + 1),
      index,
      wx * wy * wz,
    ));
  }
  SparseColMat::<usize, f32>::try_new_from_triplets(nrows, ncols, &triplets).unwrap()
}
