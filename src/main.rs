mod spline;
mod fitting;
mod data;
mod visualization;

use nalgebra::DVector;
use fitting::{build_design_matrix, least_squares_solve};
use data::generate_mock_data;
use visualization::plot_fit;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let degree = 3usize;
  let domain = (-10.0, 10.0);

  let interior_knots = vec![-8.0, -5.0, -3.0, -1.0, 0.0, 1.0, 3.0, 5.0, 8.0];
  let mut knots = vec![domain.0; degree + 1];
  knots.extend_from_slice(&interior_knots);
  knots.extend(vec![domain.1; degree + 1]);

  let (x_data, y_data) = generate_mock_data(50, domain, 500.0, 42);
  let y_vec = DVector::from_column_slice(&y_data);

  let design = build_design_matrix(&x_data, &knots, degree);
  let coeffs = least_squares_solve(&design, &y_vec);

  let n_fine = 500;
  let x_fine: Vec<f64> = (0..n_fine)
    .map(|i| domain.0 + (domain.1 - domain.0) * i as f64 / (n_fine - 1) as f64)
    .collect();

  plot_fit(&x_data, &y_data, &x_fine, &coeffs, &knots, degree, domain, "orderbook_spline_fit.png")?;
  println!("Modular prototype complete -> plot saved as 'orderbook_spline_fit.png'");

  Ok(())
}
