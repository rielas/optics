pub mod compress;
mod strip;

use crate::compress::Compressor;
use strip::StripHtml;

pub fn calculate(page_a: &str, page_b: &str) -> f64 {
    let strip_content = strip::strip_content::StripContent {};
    let stripped_a = strip_content.strip_html(page_a);
    let stripped_b = strip_content.strip_html(page_b);
    let compressor = compress::brotli::CompressBrotli::recommended();
    compressor.get_distance(&stripped_a, &stripped_b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_simple_pages() {
        let page_a = r#"<html>
    <head>
        <title>This title</title>
    </head>
    <body>
        <p class="hello">Hello, world!</p>
    </body>
</html>"#;
        let page_b = r#"<html>
    <head>
        <title>A Different Test</title>
    </head>
    <body>
        <p class="hello">Good bye, world!</p>
    </body>
</html>"#;
        assert_approx_eq!(calculate(page_a, page_b), 0.0, 0.1);
    }
}
