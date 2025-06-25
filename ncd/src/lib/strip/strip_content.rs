use super::StripHtml;
use scraper::ElementRef;

struct StripContent {}

impl StripHtml for StripContent {
    fn strip_element(&self, element: &ElementRef<'_>) -> String {
        let tag_name = element.value().name();
        let mut attributes = String::new();

        for (name, value) in element.value().attrs() {
            attributes.push_str(&format!(" {name}=\"{value}\""));
        }

        let void_element = if element.has_children() { "" } else { " /" };
        format!("<{}{}{}>", tag_name, attributes, void_element)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_page() {
        let page = r#"<html>
    <head>
        <title>Test</title>
    </head>
    <body>
        <p class="hello">Hello, world!</p>
    </body>
</html>"#;
        let stripper = StripContent {};
        let stripped = stripper.strip_html(page);
        assert_eq!(
            stripped,
            r#"<html><head><title></title></head><body><p class="hello"></p></body></html>"#
        );
    }
}
