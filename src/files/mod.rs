use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

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
            .expect("Error when compiling a regular expression.");

    if !split_regex.is_match(content) {
        // Err("Couldn't find metadata. Did you forget to add `+++`?");
    }

    let splitted_content = split_regex.captures(content).unwrap();

    Ok((
        splitted_content.get(1).unwrap().as_str().trim(),
        splitted_content.get(2).unwrap().as_str().trim(),
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

        assert_eq!(file_content_string, String::from("+++\n[metadata]\nid = \"01fbd72a-5ad4-4d4d-bc6e-7973e65e02b6\"\n+++\n\n# Lorem ipsum\n\nLorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc posuere nibh eget tortor rhoncus dictum. Lorem ipsum dolor sit amet, consectetur adipiscing elit."));
    }

    #[test]
    fn test_split_content_with_metadata() {
        let content = "+++\n[metadata]\nid = \"01fbd72a-5ad4-4d4d-bc6e-7973e65e02b6\"\n+++\n\n# Lorem ipsum\n\nLorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc posuere nibh eget tortor rhoncus dictum. Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
        let splitted_content = split_content(content).unwrap();

        assert_eq!(
            "[metadata]\nid = \"01fbd72a-5ad4-4d4d-bc6e-7973e65e02b6\"",
            splitted_content.0
        );

        assert_eq!(
            "# Lorem ipsum\n\nLorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc posuere nibh eget tortor rhoncus dictum. Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
            splitted_content.1
        );
    }

    // #[test]
    // fn test_split_content_without_metadata() {
    //     let content = "# Lorem ipsum\n\nLorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc posuere nibh eget tortor rhoncus dictum. Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
    //     let splitted_content = split_content(content).unwrap();
    // }
}
