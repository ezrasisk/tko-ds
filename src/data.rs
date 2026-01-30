use rand::prelude::*;

pub fn generate_mock_data(n_points: usize, domain: (f64, f64), noise_level: f64, seed: u64) -> (Vec<f64>, Vec<f64>) {
  let mut rng = SmallRng::seed_from_u64(seed);
  let x: Vec<f64> = (0..n_points)
    .map(|i| domain.0 + (domain.1 - domain.0) * i as f64 / (n_points - 1) as f64)
    .collect();

  let true_depth = |x: f64| 10000.0 / (1.0 + (-x / 3.0).exp());
  let y: Vec<f64> = x.iter()
    .map(|&x| true_depth(x) + noise_level * (rng.gen::<f64>() - 0.5))
    .collect();

  (x, y)
}
