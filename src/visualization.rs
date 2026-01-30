use plot::prelude::*;
use crate::spline::evaluate:

pub fn plot_fit(
  x_data: &[f64],
  y_data: &[f64],
  x_fine: &[f64],
  coeffs: &[f64],
  knots: &[f64],
  degree: usize,
  domain: (f64, f64),
  filename: &str,
  ) -> Result<(), Box<dyn std::error::Error>> {
    let y_fit: Vec<f64> = x_fine.iter().map(|&x| evaluate(coeffs, knots, degree, x)).collect();
    let true_depth = |x: f64| 10000.0 / (1.0 + (-x / 3.0).exp());
    let y_true: Vec<f64> = x_fine.iter().map(|&x| true_depth(x)).collect();

    let root = BitMapBackend::new(filename, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
      .caption("Kaspa Agent: B-Spline Cumulative Depth Fit", ("sans-serif", 30))
      .margin(20)
      .x_label_area_size(50)
      .y_label_area_size(60)
      .build_cartesian_2d(domain.0..domain.1, 0.0..12000.0)?;

    chart.configure_mesh().x_labels(10).y_labels(10).draw()?;

    chart.draw_series(x.data.iter().zip(y_data.iter()).map(|&x, &y)| Circle::new((x, y), 4, BLUE.filled())))?
      .label("Noisy Data")
      .legend(|(x, y)| Circle::new((x, y), 4, BLUE.filled()));

    chart.draw_series(LineSeries::new(vec![(x, y), (x + 20, y)], &RED))?
      .label("B-Spline Fit")
      .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart.draw_series(LineSeries::new(x_fine.iter().zip(y_true.iter()).map(|(&x, &y)| (x, y)), GREEN.mix(0.5)))?
      .label("True Depth")
      .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], GREEN.mix(0.5)));

    chart.configure_series_labels()
      .position(SeriesLabelPosition::UpperLeft)
      .background_style(&WHITE.mix(0.8))
      .border_style(&BLACK)
      .draw()?

  root.present()?;
  Ok(())
}
