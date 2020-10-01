//! This module is concerned with the content types of the source entries.
//!
//! For example: markdown, json...

use std::error::Error;
use std::fmt;

/// A content type information
#[derive(Debug, Clone, PartialEq)]
pub enum ContentType {
    Markdown,
    Other(String),
}

impl ContentType {
    /// Takes a file extension such as `md` (notice no `.`) and casts it to the matched
    /// `ContentType`.
    ///
    /// ## Errors
    ///
    /// Returns a `ContentTypeError::UnknownExtension` if the extension is not known.
    pub fn from_extension(ext: &str) -> Result<Self, ContentTypeError> {
        use ContentType::*;
        use ContentTypeError::*;

        match ext {
            "md" => Ok(Markdown),
            _ => Err(UnknownExtension(ext.into())),
        }
    }

    /// Takes a IANA mime type such as `text/markdown` and casts it to the matched
    /// `ContentType`.
    ///
    /// ## Errors
    ///
    /// Returns a `ContentTypeError::UnknownIana` if the extension is not known.
    pub fn from_iana(s: &str) -> Result<Self, ContentTypeError> {
        use ContentType::*;
        use ContentTypeError::*;

        match s {
            "text/markdown" => Ok(Markdown),
            _ => Err(UnknownIana(s.into())),
        }
    }
}

#[derive(Debug)]
pub enum ContentTypeError {
    UnknownExtension(String),
    UnknownIana(String),
}

impl fmt::Display for ContentTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ContentTypeError::*;

        match self {
            UnknownExtension(s) => write!(f, "{}", s),
            UnknownIana(s) => write!(f, "{}", s),
        }
    }
}

impl Error for ContentTypeError {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_known_extension() -> Result<(), ContentTypeError> {
        let expected = ContentType::Markdown;
        let actual = ContentType::from_extension("md")?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn parse_unknown_extension() {
        let actual = ContentType::from_extension("oo");

        assert!(actual.is_err(), "Expected an UnknownExtension error");
    }

    #[test]
    fn parse_known_iana() -> Result<(), ContentTypeError> {
        let expected = ContentType::Markdown;
        let actual = ContentType::from_iana("text/markdown")?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn parse_unknown_iana() {
        let actual = ContentType::from_iana("text/oo");

        assert!(actual.is_err(), "Expected an UnknownIana error");
    }
}
