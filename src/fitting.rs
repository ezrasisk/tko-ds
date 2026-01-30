use nalgebra::{DMatrix, DVector};
use crate::spline::basis;

pub fn build_design_matrix(x_points: &[f64], knots: &[f64], degree: usize) -> DMatrix<f64> {
  let n_basis = knots.len() - degree - 1;
  let n_points = x_points.len();
  let mut design = DMatrix::zeros(n_points, n_basis);

  for (row, &x) in x_points.iter().enumerate() {
    for j in 0..n_basis {
      design[(row, j)] = basis(knots, degree, j, x);
    }
  }
  design
}

pub fn least_squares_solve(design: &DMatrix<f64>, y: &DVector<f64>) -> DVector<f64> {
  let dt_d = design.transpose() * design;
  let dt_y = design.transpose() * y;

  dt_d.clone()
    .cholesky()
    .expect("Cholesky failed - add regularization if needed")
    .solve(&dt_y)
}
