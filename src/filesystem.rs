//! This module deals with the collection of information from files.
//!
//! In early versions only markdown files are allowed

use crate::context::Result;
// use regex::Regex;
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

/// Get all valid files from a path
pub fn get_files<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>> {
    let mut paths = vec![];

    for result in fs::read_dir(path)? {
        let path = result?.path();

        if path.is_file() {
            if is_valid_file(&path) {
                paths.push(path);
            }
        }
    }

    Ok(paths)
}

/// Get all valid files from a path and all subdirs
pub fn get_files_recursive<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>> {
    let mut paths = vec![];

    for result in fs::read_dir(path)? {
        let path = result?.path();

        if path.is_dir() {
            let recursive_paths = get_files_recursive(&path);
            paths.append(recursive_paths?.as_mut());
        }

        if path.is_file() {
            if is_valid_file(&path) {
                paths.push(path);              
            }
        }
    }

    Ok(paths)
}

/// Check if a file is a valid source
fn is_valid_file(path: &PathBuf) -> bool {
    // TODO: This array should go in other place, like context...
    let valid_extensions = ["md", "markdown"];

    if let Some(ext) = path.extension() {
        let extension = ext.to_str().unwrap();
        return valid_extensions.contains(&extension);
    }

    false
}

/// Get the content of a file as String
pub fn get_content_as_string<P: AsRef<Path>>(path: P) -> Result<String> {
    let file_content = fs::read_to_string(path)?;

    Ok(file_content)
}

/// Get the content of a file as binary
pub fn get_content_as_binary<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let file_content = fs::read(path)?;

    Ok(file_content)
}

/// Take an String with all the file content and split it in metada and data
// TODO: Move this function out of this module. Split the content should be in charge of another module
// pub fn split_content<'c>(content: &'c str) -> Result<(&'c str, &'c str)> {
//     let split_regex =
//         Regex::new(r"^[[:space:]]*\+\+\+(\r?\n(?s).*?(?-s))\+\+\+\r?\n?((?s).*(?-s))$")
//             .expect("Something went wrong when compiling a regular expression.");

//     if !split_regex.is_match(content) {
//         return Err(Box::new(SplitError(
//             "Couldn't find metadata. Did you forget to add `+++`?".into(),
//         )));
//     }

//     let captures = split_regex.captures(content).ok_or(SplitError(
//         "Something went wrong when splitting the content.".into(),
//     ))?;

//     Ok((
//         captures
//             .get(1)
//             .ok_or(SplitError("Couldn't find any metadata".into()))?
//             .as_str()
//             .trim(),
//         captures
//             .get(2)
//             .ok_or(SplitError("Couldn't find any content".into()))?
//             .as_str()
//             .trim(),
//     ))
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_valid_files() {
        let paths = get_files("test/files");
        assert_eq!(paths.unwrap().len(), 2);
    }

    #[test]
    fn get_valid_files_recursive() {
        let paths = get_files_recursive("test/files");
        assert_eq!(paths.unwrap().len(), 4);
    }

    #[test]
    fn read_content_as_string() {
        let file_content = get_content_as_string("test/files/01.md");
        let file_content_string = file_content.unwrap();

        let expected = r#"+++
id = "01"
+++

# Lorem ipsum

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc posuere nibh eget tortor rhoncus dictum. Lorem ipsum dolor sit amet, consectetur adipiscing elit."#;

        assert_eq!(file_content_string, expected.to_string());
    }
    #[test]
    fn read_content_as_binary() {
        let file_content = get_content_as_binary("test/files/01.md");
        assert!(file_content.is_ok());
    }

    //     #[test]
    //     fn split_content_with_metadata() {
    //         let content = r#"+++
    // [metadata]
    // id = "01fbd72a-5ad4-4d4d-bc6e-7973e65e02b6"
    // +++

    // # Lorem ipsum

    // Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc posuere nibh eget tortor rhoncus dictum. Lorem ipsum dolor sit amet, consectetur adipiscing elit."#;
    //         let content_split = split_content(content).unwrap();

    //         assert_eq!(
    //             "[metadata]\nid = \"01fbd72a-5ad4-4d4d-bc6e-7973e65e02b6\"",
    //             content_split.0
    //         );

    //         assert_eq!(
    //             "# Lorem ipsum\n\nLorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc posuere nibh eget tortor rhoncus dictum. Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
    //             content_split.1
    //         );
    //     }

    //     #[test]
    //     fn split_content_with_empty_metadata() {
    //         let content = r#"+++
    // +++

    // # Lorem ipsum

    // Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc posuere nibh eget tortor rhoncus dictum. Lorem ipsum dolor sit amet, consectetur adipiscing elit."#;
    //         assert_eq!(split_content(content).is_ok(), true);
    //     }

    //     #[test]
    //     fn split_content_without_metadata() {
    //         let content = r#"
    // # Lorem ipsum

    // Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc posuere nibh eget tortor rhoncus dictum. Lorem ipsum dolor sit amet, consectetur adipiscing elit."#;

    //         assert_eq!(split_content(content).is_err(), true);
    //     }
}
