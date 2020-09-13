use std::{error::Error};
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
        if ext != "md" {
            return false;
        }        
    }

    true
}

/// Get the content of a file as String
pub fn get_file_content<P: AsRef<Path>>(path: P) -> Result<String> {
    let file = fs::read_to_string(path)?;

    Ok(file)
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

        assert_eq!(file_content_string, String::from("+++\n+++\n\n# Lorem ipsum\n\nLorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc posuere nibh eget tortor rhoncus dictum. Lorem ipsum dolor sit amet, consectetur adipiscing elit."));
    }
}
