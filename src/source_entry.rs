//! This module is concerned with the source entries.

use crate::content::Checksum;
use crate::content_type::{ContentType, ContentTypeError};
use crate::source::{Id as SourceId, ParseIdError};
use std::error::Error;
use std::fmt;
use std::str::FromStr;

/// A source entry.
///
/// A source entry is the unprocessed information obtained from, for example, a file found in a
/// source directory.
#[derive(Debug, PartialEq)]
pub struct SourceEntry {
    /// The local path to the original file.
    id: String,
    source_id: SourceId,
    content_id: Option<Checksum>,
    content_type: ContentType,
}

impl FromStr for SourceEntry {
    type Err = SourceEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair: Vec<&str> = s.splitn(2, ':').collect();

        if pair.len() != 2 {
            return Err(SourceEntryError::UnknownPattern);
        }

        let id = pair[1].to_string();
        let source_id = pair[0].parse()?;
        let content_type = if let Some(ext) = id.rsplitn(2, '.').collect::<Vec<&str>>().first() {
            ContentType::from_extension(ext)?
        } else {
            return Err(SourceEntryError::MissingExtension);
        };

        let entry = SourceEntry {
            id,
            source_id,
            content_id: None,
            content_type,
        };

        Ok(entry)
    }
}

#[derive(Debug)]
pub enum SourceEntryError {
    ContentType(ContentTypeError),
    SourceId(ParseIdError),
    MissingExtension,
    UnknownPattern,
    Unknown(String),
}

impl fmt::Display for SourceEntryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SourceEntryError::ContentType(err) => write!(f, "{}", err),
            SourceEntryError::SourceId(err) => write!(f, "{}", err),
            SourceEntryError::MissingExtension => {
                write!(f, "Couldn't find an extension to identify the content type")
            }
            SourceEntryError::UnknownPattern => {
                write!(f, "Qualified identifiers must have the form `source:path`")
            }
            SourceEntryError::Unknown(err) => write!(f, "{}", err),
        }
    }
}

impl Error for SourceEntryError {}

impl From<ParseIdError> for SourceEntryError {
    fn from(err: ParseIdError) -> SourceEntryError {
        SourceEntryError::SourceId(err)
    }
}

impl From<ContentTypeError> for SourceEntryError {
    fn from(err: ContentTypeError) -> SourceEntryError {
        SourceEntryError::ContentType(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_source_entry() -> Result<(), SourceEntryError> {
        let expected = SourceEntry {
            id: "foo.md".into(),
            source_id: "unnamed".parse()?,
            content_id: None,
            content_type: ContentType::Markdown,
        };

        let actual = SourceEntry::from_str("unnamed:foo.md")?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn source_entry_empty_source() -> Result<(), SourceEntryError> {
        let expected = SourceEntry {
            id: "foo.md".into(),
            source_id: "".parse()?,
            content_id: None,
            content_type: ContentType::Markdown,
        };

        let actual = SourceEntry::from_str(":foo.md")?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn source_entry_without_source() {
        let actual = SourceEntry::from_str("foo.md");

        assert!(actual.is_err(), "Expected error given missing source");
    }
}
