//! Cache storage
//!
//! This module contains the cache implementation and helper functions.

use super::context::Result;
use rusqlite::Connection;
use std::include_str;
use std::path::Path;

/// Opens a SQLite database at the given path.
pub fn connect(path: &Path) -> Result<Connection> {
    let conn = Connection::open(path)?;
    // conn.pragma_update(None, "foreign_keys", &"off")?;
    conn.pragma_update(None, "journal_mode", &"wal")?;

    Ok(conn)
}

/// WAL persists across connections, this ensures it is switched off.
pub fn disconnect(conn: &Connection) -> Result<()> {
    conn.pragma_update(None, "wal_checkpoint", &"restart")?;
    conn.pragma_update(None, "journal_mode", &"delete")?;

    Ok(())
}

/// Sets up the cache schema.
pub fn bootstrap(conn: &Connection) -> Result<()> {
    let bootstrap = include_str!("./sql/bootstrap.sql");

    conn.execute_batch(&bootstrap)?;

    Ok(())
}
