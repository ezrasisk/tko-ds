pub fn basis(knots: &[f64], degree: usize, i: usize, x: f64) -> f64 {
  if degree == 0 {
    if knots[i] <= x && x  knots[i + 1] {
      1.0
    } else {
      0.0
    }
  } else {
    let denom1 = knots[i + degree] - knots[i];
    let denom2 = knots[i + degree + 1] - knots[i + 1];
    let a1 = if denom1 > 1e-10 { (x - knots[i]) / denom1 } else { 0.0 };
    let a2 = if denom2 > 1e-10 { (knots[i + degree + 1] - x) / denom2 } else { 0.0 };
    a1 * basis(knots, degree - 1, i, x) + a2 * basis(knots, degree - 1, i + 1, x)
  }
}

pub fn evaluate(coeffs: &[f64], knots: &[f64], degree: usize, x: f64) -> f64 {
  let n_basis = coeffs.len();
  let mut value = 0.0;
  for j in 0..n_basis {
    value += coeffs[j] * basis(knots, degree, j, x);
  } 
  value
}
