//! This module contains helpers for managing contextual information to be tracked throught the
//! process.

use clap::crate_version;
use std::error::Error;
use std::result::Result as StdResult;

/// The message delivered to the user.
///
/// This value is typically used in CLI commands to communicate successful outcomes to the user.
pub type Message = String;

/// Tidy result alias.
pub type Result<T, E = Box<dyn Error>> = StdResult<T, E>;

/// A context information
///
/// Has as many members as needed as a key:value pair
#[derive(Debug)]
struct Context {
    version: String,
    // created: DateTime<Utc>, //TODO: Waiting to merge with Arnau code because he will add crono dependency and to prevent conflicts
}

/// Context struct implementation
impl Context {
    /// Create a Context variable with default values
    pub fn new() -> Self {
        Context {
            version: crate_version!().to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    mod context {
        use super::super::*;

        #[test]
        fn test_create_a_default_context() {
            let context = Context::new();

            assert_eq!(context.version, crate_version!().to_string());
        }
    }
}
