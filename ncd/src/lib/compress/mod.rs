mod compress;

trait Calculator {
    fn calculate(&self, page_a: &str, page_b: &str) -> f64;
}
