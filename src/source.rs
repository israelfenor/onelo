//! This module is concerned with the data sources.

use chrono::prelude::*;
use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// A data source.
///
/// Identifiers serve as namespaces in `SourceEntry` and in MarkDown links when href attributes are
/// of the form `source_id:path`.
#[derive(Debug, Clone, PartialEq)]
pub struct Source {
    id: Id,
    route: PathBuf,
    // TODO: Should align with a Git commit hash. Consider how accommodating we
    // can be with other VCS such as fossil or mercurial.
    checksum: Option<Vec<u8>>,
    timestamp: DateTime<Utc>,
}

impl Source {
    pub fn new<P: AsRef<Path>>(id: Id, route: P) -> Self {
        Source {
            id,
            route: PathBuf::from(route.as_ref()),
            checksum: None,
            timestamp: Utc::now(),
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }
}

/// A source identifier.
///
/// A source identifier must obey certain rules such as not having a `:` in it.
#[derive(Debug, Clone, PartialEq)]
pub struct Id(String);

impl FromStr for Id {
    type Err = ParseIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        guard_against_char(s, ':')?;

        Ok(Id(s.into()))
    }
}

/// An error which can be returned when parsing a source identifier.
#[derive(Debug)]
pub struct ParseIdError(String);

impl fmt::Display for ParseIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ParseIdError {}

/// Guards againts the presence of a character in the given string.
fn guard_against_char(s: &str, ch: char) -> Result<(), ParseIdError> {
    match s.find(ch) {
        None => Ok(()),
        Some(_) => Err(ParseIdError(format!(
            "Source identifiers cannot have `{}` in them",
            ch
        ))),
    }
}

#[cfg(test)]
mod tests {
    mod source {
        use super::super::*;
        use crate::context::Result;

        #[test]
        fn baseline() -> Result<()> {
            let id = Id::from_str("foo")?;
            let actual = Source::new(id.clone(), "/foo/bar");

            assert_eq!(actual.id(), &id);

            Ok(())
        }
    }

    mod id {
        use super::super::*;

        #[test]
        fn succeeds() {
            let input = Id::from_str("foo");

            assert!(input.is_ok(), "Expects a string without `:`");
        }

        #[test]
        fn fails() {
            let input = Id::from_str("broken:input");

            assert!(input.is_err(), "Expects a string with `:`");
        }
    }

    mod check_char {
        use super::super::*;

        #[test]
        fn succeeds() {
            let input = "foo";

            assert!(
                guard_against_char(input, ':').is_ok(),
                "Expects a string without `:`"
            );
        }

        #[test]
        fn fails() {
            let input = "broken:input";

            assert!(
                guard_against_char(input, ':').is_err(),
                "Expects a string with `:`"
            );
        }
    }
}
