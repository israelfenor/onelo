use std::{error::Error, io::BufReader};
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

// pub fn process_file<P: AsRef<Path>>(path: P) -> Result<String> {
//     let file = fs::File::open("test/files/01.md")?;
//     let reader = BufReader::new(file);
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_only_md_files() {
        let paths = get_markdown_files("test/files");
        assert_eq!(paths.unwrap().len(), 2);
    }

    // #[test]
    // fn test_read_markdown_file() {
    //     read_markdown_file("test/files/01.md");
    //     assert!(true);
    // }
}
