//! This module contains helpers for managing contextual information to be tracked throught the
//! process.

use std::error::Error;
use std::result::Result as StdResult;

/// The message delivered to the user.
///
/// This value is typically used in CLI commands to communicate successful outcomes to the user.
pub type Message = String;

/// Tidy result alias.
pub type Result<T, E = Box<dyn Error>> = StdResult<T, E>;
