use benchmark::dataset;
use core::calculate;
use core::compress::{brotli::CompressBrotli, Compressor};
use plotly::common::{AxisSide, Title};
use plotly::layout::Axis;
use plotly::{HeatMap, Layout, Plot};
use rayon::prelude::*;
use std::path::PathBuf;
use std::time::{Duration, Instant};

fn get_dataset_path(directory: &str) -> PathBuf {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let full_path = std::path::Path::new(project_root)
        .join("../../dataset")
        .join(directory);
    full_path
}

fn read_from_file(file_path: &str) -> String {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let full_path = std::path::Path::new(project_root).join(file_path);
    std::fs::read_to_string(full_path).expect("Failed to read file")
}

const FILE_PATH: &str = "../../dataset/imdb/list/ls541382956/?ref_=tt_urls_2.html";

#[derive(Debug, Clone)]
struct BenchmarkResult {
    quality: u32,
    lg_window_size: u32,
    compression_ratio: f64,
    duration: Duration,
}

fn same_page() {
    println!("A distance betwwen the same pages should be close to 0.0");
    let page_html = read_from_file(FILE_PATH);
    let mut results = Vec::new();

    for quality in 3..11 {
        for lg_window_size in 20..=22 {
            let start = Instant::now();
            let compressor = CompressBrotli::new(quality, lg_window_size);
            let result = compressor.get_distance(&page_html, &page_html);
            let duration = start.elapsed();

            let benchmark_result = BenchmarkResult {
                quality,
                lg_window_size,
                compression_ratio: result,
                duration,
            };

            results.push(benchmark_result.clone());

            println!(
                "Quality: {quality}, LG Window Size: {lg_window_size}, Distance: {:.6}, Time: {:?}",
                result, duration
            );
        }
    }
}

fn heatmap(dataset_name: &str) {
    let dataset =
        dataset::Dataset::new(get_dataset_path(dataset_name)).expect("Failed to load dataset");
    let entries = dataset.entries();
    let page_names = entries
        .iter()
        .map(|entry| entry.get_name())
        .collect::<Vec<String>>();

    let heatmap = HeatMap::new(
        page_names.clone(),
        page_names.clone(),
        entries
            .par_iter()
            .map(|entry_a| {
                entries
                    .iter()
                    .map(|entry_b| {
                        println!(
                            "Calculating distance between {} and {}",
                            entry_a.url, entry_b.url
                        );
                        calculate(
                            &entry_a.get_content().unwrap(),
                            &entry_b.get_content().unwrap(),
                        )
                    })
                    .collect::<Vec<f64>>()
            })
            .collect::<Vec<Vec<f64>>>(),
    );

    let mut plot = Plot::new();
    plot.add_trace(heatmap);

    let layout = Layout::new()
        .title(format!("Normalized Compression Distance for {}", dataset_name))
        .width(800)
        .height(800)
        .x_axis(
            Axis::new()
                .title(Title::with_text("Page A"))
                .side(AxisSide::Bottom)
                .auto_margin(true)
                .tick_angle(-90.0)
                .tick_text(page_names.clone()),
        )
        .y_axis(
            Axis::new()
                .title("Page B")
                .scale_anchor("x")
                .auto_margin(true)
                .tick_text(page_names.clone()),
        );
    plot.set_layout(layout);

    plot.show();
}

fn main() {
    println!("NCD Brotli Benchmark");
    //same_page();
    heatmap("euronews.com");
}
