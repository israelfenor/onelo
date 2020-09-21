//! This module is concerned with the content types of the sources
//!
//! For example: markdown, json...

/// A content type information
#[derive(Debug)]
pub enum ContentType {
    Markdown,
    Other(String),
}

#[cfg(test)]
mod test {
    mod content_type {
        use super::super::*;

        #[test]
        fn test_create_a_content_type_variable() {
            let md_content_type = ContentType {
                id: "text/markdown".to_string(),
            };

            assert_eq!(md_content_type.id, "text/markdown".to_string());
        }
    }
}
