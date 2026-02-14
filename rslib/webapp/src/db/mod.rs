use anyhow::Result;
use rusqlite::{params, Connection};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub mod sessions;
pub mod users;

pub use sessions::{Session, SessionStore};
pub use users::{User, UserStore};

const SCHEMA_SQL: &str = include_str!("schema.sql");

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute("PRAGMA foreign_keys = ON", [])?;
        Ok(Self { conn })
    }

    pub fn initialize(&self) -> Result<()> {
        self.conn.execute_batch(SCHEMA_SQL)?;
        Ok(())
    }

    pub fn users(&self) -> UserStore<'_> {
        UserStore::new(&self.conn)
    }

    pub fn sessions(&self) -> SessionStore<'_> {
        SessionStore::new(&self.conn)
    }

    pub fn cleanup_expired_sessions(&self) -> Result<usize> {
        let now = current_timestamp();
        let count = self.conn.execute(
            "DELETE FROM sessions WHERE expires_at < ?1",
            params![now],
        )?;
        Ok(count)
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
        let table_count: i64 = db
            .conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('users', 'sessions', 'schema_version')",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(table_count, 3);
    }

    #[test]
    fn test_cleanup_expired_sessions() {
        let db = Database::open(":memory:").unwrap();
        db.initialize().unwrap();

        // Insert expired session
        db.conn
            .execute(
                "INSERT INTO users (username, password_hash, created_at, updated_at) VALUES ('test', 'hash', 0, 0)",
                [],
            )
            .unwrap();

        db.conn
            .execute(
                "INSERT INTO sessions (id, user_id, created_at, expires_at, last_accessed) VALUES ('expired', 1, 0, 0, 0)",
                [],
            )
            .unwrap();

        let count = db.cleanup_expired_sessions().unwrap();
        assert_eq!(count, 1);
    }
}
