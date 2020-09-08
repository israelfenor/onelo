use std::{path::{PathBuf, self}, error::Error};

fn main() {
    let dir = path::Path::new("test/files");
    let paths = get_markdown_files(dir.to_path_buf());

    dbg!(paths);
}

fn get_markdown_files(dir: PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut paths = vec![];

    for result in dir.read_dir()? {
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
    fn test_get_only_md_files () {
        let dir = path::Path::new("test/files");
        let paths = get_markdown_files(dir.to_path_buf());
        assert_eq!(paths.unwrap().len(), 2);
    }
}