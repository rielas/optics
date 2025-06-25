use crate::compress::Calculator;

use std::{
    cmp::{self},
    io::{BufWriter, Write},
};

const BUFFER_SIZE: usize = 4096;
const QUALITY: u32 = 5;
const LG_WINDOW_SIZE: u32 = 21;

struct CompressBrotli {}

impl Calculator for CompressBrotli {
    fn calculate(&self, page_a: &str, page_b: &str) -> f64 {
        let length_combined = {
            let length_combined_a_b = get_compressed_size(
                &(page_a.to_owned() + page_b),
                BUFFER_SIZE,
                QUALITY,
                LG_WINDOW_SIZE,
            );

            let length_combined_b_a = get_compressed_size(
                &(page_b.to_owned() + page_a),
                BUFFER_SIZE,
                QUALITY,
                LG_WINDOW_SIZE,
            );
            cmp::min(length_combined_a_b, length_combined_b_a)
        };

        let a_compressed = get_compressed_size(page_a, BUFFER_SIZE, QUALITY, LG_WINDOW_SIZE);
        let b_compressed = get_compressed_size(page_b, BUFFER_SIZE, QUALITY, LG_WINDOW_SIZE);

        let min = cmp::min(a_compressed, b_compressed);
        let max = cmp::max(a_compressed, b_compressed);

        (length_combined - min) as f64 / max as f64
    }
}

fn get_compressed_size(buf: &str, buffer_size: usize, quality: u32, lg_window_size: u32) -> usize {
    let mut out = BufWriter::new(Vec::new());

    {
        let mut writer =
            brotli::CompressorWriter::new(&mut out, buffer_size, quality, lg_window_size);
        writer.write_all(buf.as_bytes()).unwrap();
    }

    out.into_inner().unwrap().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_brotli() {
        let compressor = CompressBrotli {};
        let page_a = "<html><body>Hello, world!</body></html>";
        let page_b = "<html><body>Hello, world!</body></html>";
        let result = compressor.calculate(page_a, page_b);
        assert!(result <= 1.0);
    }
}
