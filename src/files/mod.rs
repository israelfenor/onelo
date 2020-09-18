use crate::context::Result;
use regex::Regex;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct SplitError(String);

impl fmt::Display for SplitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for SplitError {}

/// Get all markdown files of a path
pub fn get_markdown_files<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>> {
    let mut paths = vec![];

    for result in fs::read_dir(path)? {
        let path = result?.path();

        if is_valid_file(&path) {
            paths.push(path);
        }
    }

    Ok(paths)
}

/// Check if a file is a valid source
fn is_valid_file(path: &PathBuf) -> bool {
    if let Some(ext) = path.extension() {
        return ext == "md";
    }

    false
}

/// Get the content of a file as String
pub fn get_file_content<P: AsRef<Path>>(path: P) -> Result<String> {
    let file = fs::read_to_string(path)?;

    Ok(file)
}

/// Take an String with all the file content and split it in metada and data
pub fn split_content<'c>(content: &'c str) -> Result<(&'c str, &'c str)> {
    let split_regex =
        Regex::new(r"^[[:space:]]*\+\+\+(\r?\n(?s).*?(?-s))\+\+\+\r?\n?((?s).*(?-s))$")
            .expect("Something went wrong when compiling a regular expression.");

    if !split_regex.is_match(content) {
        return Err(Box::new(SplitError(
            "Couldn't find metadata. Did you forget to add `+++`?".into(),
        )));
    }

    let captures = split_regex.captures(content).ok_or(SplitError(
        "Something went wrong when splitting the content.".into(),
    ))?;

    Ok((
        captures
            .get(1)
            .ok_or(SplitError("Couldn't find any metadata".into()))?
            .as_str()
            .trim(),
        captures
            .get(2)
            .ok_or(SplitError("Couldn't find any content".into()))?
            .as_str()
            .trim(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_only_md_files() {
        let paths = get_markdown_files("test/files");
        assert_eq!(paths.unwrap().len(), 2);
    }

    #[test]
    fn test_get_markdown_file_content() {
        let file_content = get_file_content("test/files/01.md");
        let file_content_string = file_content.unwrap();

        let expected = r#"+++
[metadata]
id = "01fbd72a-5ad4-4d4d-bc6e-7973e65e02b6"
+++

# Lorem ipsum

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc posuere nibh eget tortor rhoncus dictum. Lorem ipsum dolor sit amet, consectetur adipiscing elit."#;

        assert_eq!(file_content_string, expected.to_string());
    }

    #[test]
    fn test_split_content_with_metadata() {
        let content = r#"+++
[metadata]
id = "01fbd72a-5ad4-4d4d-bc6e-7973e65e02b6"
+++

# Lorem ipsum

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc posuere nibh eget tortor rhoncus dictum. Lorem ipsum dolor sit amet, consectetur adipiscing elit."#;
        let content_split = split_content(content).unwrap();

        assert_eq!(
            "[metadata]\nid = \"01fbd72a-5ad4-4d4d-bc6e-7973e65e02b6\"",
            content_split.0
        );

        assert_eq!(
            "# Lorem ipsum\n\nLorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc posuere nibh eget tortor rhoncus dictum. Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
            content_split.1
        );
    }

    #[test]
    fn test_split_content_with_empty_metadata() {
        let content = r#"+++
+++

# Lorem ipsum

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc posuere nibh eget tortor rhoncus dictum. Lorem ipsum dolor sit amet, consectetur adipiscing elit."#;
        assert_eq!(split_content(content).is_ok(), true);
    }

    #[test]
    fn test_split_content_without_metadata() {
        let content = r#"
# Lorem ipsum

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc posuere nibh eget tortor rhoncus dictum. Lorem ipsum dolor sit amet, consectetur adipiscing elit."#;

        assert_eq!(split_content(content).is_err(), true);
    }
}
