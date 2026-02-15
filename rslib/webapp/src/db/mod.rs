use std::path::Path;
use std::sync::Mutex;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use anyhow::Result;
use rusqlite::params;
use rusqlite::Connection;

pub mod sessions;
pub mod users;

pub use sessions::Session;
pub use sessions::SessionStore;
pub use users::User;
pub use users::UserStore;

const SCHEMA_SQL: &str = include_str!("schema.sql");

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute("PRAGMA foreign_keys = ON", [])?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn initialize(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(SCHEMA_SQL)?;
        Ok(())
    }

    pub fn users(&self) -> UserStore<'_> {
        UserStore::new(self)
    }

    pub fn sessions(&self) -> SessionStore<'_> {
        SessionStore::new(self)
    }

    pub fn cleanup_expired_sessions(&self) -> Result<usize> {
        let now = current_timestamp();
        let conn = self.conn.lock().unwrap();
        let count = conn.execute("DELETE FROM sessions WHERE expires_at < ?1", params![now])?;
        Ok(count)
    }

    pub(crate) fn with_conn<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&Connection) -> Result<R>,
    {
        let conn = self.conn.lock().unwrap();
        f(&conn)
    }
}

pub fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_initialization() {
        let db = Database::open(":memory:").unwrap();
        db.initialize().unwrap();

        // Verify tables exist
        let table_count: i64 = db.with_conn(|conn| {
            conn.query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('users', 'sessions', 'schema_version')",
                [],
                |row| row.get(0),
            )
            .map_err(Into::into)
        }).unwrap();
        assert_eq!(table_count, 3);
    }

    #[test]
    fn test_cleanup_expired_sessions() {
        let db = Database::open(":memory:").unwrap();
        db.initialize().unwrap();

        // Insert expired session
        db.with_conn(|conn| {
            conn.execute(
                "INSERT INTO users (username, password_hash, created_at, updated_at) VALUES ('test', 'hash', 0, 0)",
                [],
            )?;
            conn.execute(
                "INSERT INTO sessions (id, user_id, created_at, expires_at, last_accessed) VALUES ('expired', 1, 0, 0, 0)",
                [],
            )?;
            Ok(())
        }).unwrap();

        let count = db.cleanup_expired_sessions().unwrap();
        assert_eq!(count, 1);
    }
}
