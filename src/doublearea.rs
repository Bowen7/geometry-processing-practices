use faer::Mat;

pub fn doublearea(l: &Mat<f32>) -> Vec<f32> {
  let mut double_areas = Vec::<f32>::with_capacity(l.nrows());
  for i in 0..l.nrows() {
    let a = l[(i, 0)];
    let b = l[(i, 1)];
    let c = l[(i, 2)];
    double_areas.push(2.0 * 0.25 * ((a + b + c) * (-a + b + c) * (a - b + c) * (a + b - c)).sqrt());
  }
  double_areas
}

#[cfg(test)]
mod tests {
  use super::*;
  use faer::mat;

  #[test]
  fn test_doublearea() {
    let l = mat![[3.0, 4.0, 5.0]];
    let double_areas = doublearea(&l);
    assert_eq!(double_areas, vec![12.0]);
  }
}
