//! This module contains helpers for managing contextual information to be tracked throught the
//! process.

use chrono::prelude::*;
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
pub struct Context {
    version: String,        // The verion of onelo used
    created: DateTime<Utc>, // When the context is created
}

/// Context struct implementation
impl Context {
    /// Create a Context variable with default values
    pub fn new() -> Self {
        Context {
            version: crate_version!().to_string(),
            created: Utc::now(),
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
            assert!(context.created.to_string().len() > 0);
        }
    }
}
