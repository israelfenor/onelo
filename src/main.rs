use std::{fs, ffi::OsStr};

fn main() {
    get_markdown_files("test/files".to_string());
}

fn get_markdown_files (path: String) -> Vec<String> {
    let mut markdown_files: Vec<String> = vec![];
    
    match fs::read_dir(path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(files) => for file in files {
            match file {
                Err(_) => {}
                Ok(_) => {
                    if file.unwrap().path().extension().and_then(OsStr::to_str) == Some("md") {
                        println!("> MD");
                        //markdown_files.push();
                    }
                }
            }
        }
    }

    markdown_files
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_only_md_files () {
        let markdown_files = get_markdown_files("test/files".to_string());
        assert_eq!(markdown_files.len(), 2);
    }
}