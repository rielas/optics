pub mod dataset;

extern crate plotters;

use plotters::evcxr::SVGWrapper;
use plotters::prelude::*;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub quality: u32,
    pub lg_window_size: u32,
    pub compression_ratio: f64,
    pub duration: Duration,
}

#[allow(dead_code)]
pub fn point_series(results: &[BenchmarkResult]) -> SVGWrapper {
    evcxr_figure((800, 600), |root| {
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .caption(
                "NCD Brotli: distance and compression time",
                ("sans-serif", 40),
            )
            .margin(10)
            .x_label_area_size(50)
            .y_label_area_size(60)
            .build_cartesian_2d(
                (results
                    .iter()
                    .map(|r| r.duration.as_secs_f64())
                    .filter(|&x| x > 0.0)
                    .fold(f64::INFINITY, f64::min)
                    * 0.9
                    ..results
                        .iter()
                        .map(|r| r.duration.as_secs_f64())
                        .fold(0.0, f64::max)
                        * 1.1)
                    .log_scale(),
                (results
                    .iter()
                    .map(|r| r.compression_ratio)
                    .filter(|&x| x > 0.0)
                    .fold(f64::INFINITY, f64::min)
                    * 0.9
                    ..results
                        .iter()
                        .map(|r| r.compression_ratio)
                        .fold(0.0, f64::max)
                        * 1.1)
                    .log_scale(),
            )?;

        chart
            .configure_mesh()
            .x_desc("Compression Time (seconds). The lower, the better")
            .y_desc("Distance. The lower, the better")
            .draw()?;

        for result in results {
            let color = match result.quality {
                0..=2 => &RED,
                3..=5 => &BLUE,
                6..=8 => &GREEN,
                _ => &MAGENTA,
            };

            chart
                .draw_series(PointSeries::of_element(
                    vec![(result.duration.as_secs_f64(), result.compression_ratio)],
                    5,
                    color,
                    &|c, s, st| {
                        EmptyElement::at(c)
                            + Circle::new((0, 0), s, st.filled())
                            + Text::new(
                                format!("{}, {}", result.lg_window_size, result.quality),
                                (0, 10),
                                ("sans-serif", 10),
                            )
                    },
                ))?
                .label(format!("Q{}", result.quality))
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], color));
        }

        chart.configure_mesh().draw()?;
        Ok(())
    })
}
