//! Cache storage
//!
//! This module contains the cache implementation and helper functions.

use super::context::Result;
use rusqlite::Connection;
use std::include_str;
use std::path::Path;

/// Opens a SQLite database at the given path.
pub fn connect<P: AsRef<Path>>(path: P) -> Result<Connection> {
    let conn = Connection::open(path)?;
    // conn.pragma_update(None, "foreign_keys", &"off")?;
    conn.pragma_update(None, "journal_mode", &"wal")?;

    Ok(conn)
}

/// WAL persists across connections, this ensures it is switched off.
pub fn clean(conn: &Connection) -> Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::NO_PARAMS;

    #[test]
    fn bootstrap_cache() -> Result<()> {
        let mut conn = connect(":memory:")?;

        bootstrap(&conn)?;

        let tx = conn.transaction()?;
        let mut stmt = tx.prepare(
            r#"
            SELECT
                name
            FROM
                sqlite_schema
            WHERE
                type = 'table'
            ORDER BY 1;
            "#,
        )?;
        let mut actual: Vec<String> = Vec::new();
        let rows = stmt.query_map(NO_PARAMS, |row| row.get(0))?;

        for row in rows {
            actual.push(row?);
        }

        assert_eq!(actual.len(), 5);

        Ok(())
    }
}
