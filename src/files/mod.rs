use std::path::{Path, PathBuf};
use std::fs;
use std::error::Error;

type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

pub fn get_markdown_files<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>> {
    let mut paths = vec![];

    for result in fs::read_dir(path)? {
        let path = result?.path();

        if let Some(ext) = path.extension() {
            if ext == "md" {
                paths.push(path);
            }
        }
    }

    Ok(paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_only_md_files() {
        let paths = get_markdown_files("test/files");
        assert_eq!(paths.unwrap().len(), 2);
    }
}