use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct Entry {
    pub url: String,
    pub page_type: String,
    pub filepath: PathBuf,
}

impl Entry {
    pub fn get_content(&self) -> io::Result<String> {
        fs::read_to_string(&self.filepath)
    }

    pub fn get_name(&self) -> String {
        let without_protocol = self.url.split("://").nth(1).unwrap_or_default();
        let without_domain = without_protocol
            .split('/')
            .skip(1)
            .collect::<Vec<&str>>()
            .join("/");
        const MAX_LENGTH: usize = 10;
        let truncated = without_domain
            .split('/')
            .map(|part| {
                if part.len() > MAX_LENGTH {
                    format!("{}…", &part[..MAX_LENGTH])
                } else {
                    part.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("/");

        "/".to_owned() + &truncated + " " + &self.icon()
    }

    fn icon(&self) -> String {
        let pt = self.page_type.trim().to_lowercase();
        match pt.as_str() {
            "article" => "📄".to_string(),
            "user" | "person" | "profile" => "👤".to_string(),
            "category" | "section" => "📂".to_string(),
            "disambiguation" => "🤔".to_string(),
            "search" => "🔍".to_string(),
            "product" => "🛍️".to_string(),
            "store" => "🏬".to_string(),
            "movie" => "🎬".to_string(),
            "user_list" => "📋".to_string(),
            "video" => "📹".to_string(),
            "news_article" => "📰".to_string(),
            "tag" => "🏷️".to_string(),
            "special" => "✨".to_string(),
            _ => panic!("Unknown page type: {}", pt),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dataset {
    entries: Vec<Entry>,
    directory: PathBuf,
}

impl Dataset {
    pub fn new<P: AsRef<Path>>(directory: P) -> io::Result<Self> {
        let mut dataset = Dataset {
            entries: Vec::new(),
            directory: PathBuf::from(directory.as_ref()),
        };
        dataset.load_csv(dataset.directory.join("dataset.csv"))?;
        Ok(dataset)
    }

    fn load_csv<P: AsRef<Path>>(&mut self, csv_path: P) -> io::Result<()> {
        let file = fs::File::open(csv_path)?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        lines.next();

        for line in lines {
            let line = line?;

            if let Some((url, page_type)) = self.parse_csv_line(&line) {
                if let Some(filepath) = self.get_file_path(&url) {
                    self.entries.push(Entry {
                        url,
                        page_type,
                        filepath,
                    });
                }
            }
        }

        Ok(())
    }

    fn parse_csv_line(&self, line: &str) -> Option<(String, String)> {
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() >= 2 {
            let url = parts[0].trim().trim_matches('"').to_string();
            let page_type = parts[1].trim().trim_matches('"').to_string();
            Some((url, page_type))
        } else {
            None
        }
    }

    fn url_to_filename(&self, url: &str) -> Option<String> {
        if let Some(path_start) = url.find("://").and_then(|i| url[i + 3..].find('/')) {
            let path = &url[url.find("://")? + 3 + path_start + 1..];
            let decoded = urlencoding::decode(path).ok()?;
            Some(decoded.into_owned())
        } else {
            None
        }
    }

    pub fn get_file_path(&self, url: &str) -> Option<PathBuf> {
        if let Some(page_name) = self.url_to_filename(url) {
            let extensions = [".txt", ".md", ".html", ".wiki", ""];

            for ext in &extensions {
                let filename = format!("{}{}", page_name, ext);
                let filepath = self.directory.join(&filename);

                if filepath.is_dir() {
                    let with_index = filepath.join("index.html");

                    if with_index.exists() {
                        return Some(with_index);
                    }
                }

                if filepath.exists() {
                    return Some(filepath);
                }
            }
        }

        None
    }

    pub fn get_page_type(&self, url: &str) -> Option<&String> {
        self.entries
            .iter()
            .find(|entry| entry.url == url)
            .map(|entry| &entry.page_type)
    }

    pub fn entries(&self) -> &Vec<Entry> {
        &self.entries
    }
}

#[cfg(test)]
mod tests {
    use plotters::data;

    use super::*;

    fn get_dataset_path(directory: &str) -> PathBuf {
        let project_root = env!("CARGO_MANIFEST_DIR");
        let full_path = std::path::Path::new(project_root)
            .join("../../dataset")
            .join(directory);
        full_path
    }

    #[test]
    fn test_get_file_index() {
        let dataset = Dataset::new(get_dataset_path("euronews")).unwrap();
        let entries = dataset.entries();

        for entry in entries {
            assert!(
                entry.get_content().is_ok(),
                "Failed to read content for {}",
                entry.url
            );
        }
    }
}
