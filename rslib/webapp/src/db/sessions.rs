use anyhow::Result;
use rusqlite::{params, Connection, OptionalExtension, Row};
use serde::{Deserialize, Serialize};

use super::current_timestamp;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub user_id: i64,
    pub created_at: i64,
    pub expires_at: i64,
    pub last_accessed: i64,
}

impl Session {
    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Session {
            id: row.get(0)?,
            user_id: row.get(1)?,
            created_at: row.get(2)?,
            expires_at: row.get(3)?,
            last_accessed: row.get(4)?,
        })
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at < current_timestamp()
    }
}

pub struct SessionStore<'a> {
    conn: &'a Connection,
}

impl<'a> SessionStore<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn create(&self, session_id: &str, user_id: i64, ttl_seconds: i64) -> Result<Session> {
        let now = current_timestamp();
        let expires_at = now + ttl_seconds;

        self.conn.execute(
            "INSERT INTO sessions (id, user_id, created_at, expires_at, last_accessed) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![session_id, user_id, now, expires_at, now],
        )?;

        Ok(Session {
            id: session_id.to_string(),
            user_id,
            created_at: now,
            expires_at,
            last_accessed: now,
        })
    }

    pub fn get(&self, session_id: &str) -> Result<Option<Session>> {
        self.conn
            .query_row(
                "SELECT id, user_id, created_at, expires_at, last_accessed FROM sessions WHERE id = ?1",
                params![session_id],
                Session::from_row,
            )
            .optional()
            .map_err(Into::into)
    }

    pub fn update_access_time(&self, session_id: &str) -> Result<()> {
        let now = current_timestamp();
        self.conn.execute(
            "UPDATE sessions SET last_accessed = ?1 WHERE id = ?2",
            params![now, session_id],
        )?;
        Ok(())
    }

    pub fn delete(&self, session_id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM sessions WHERE id = ?1", params![session_id])?;
        Ok(())
    }

    pub fn delete_by_user(&self, user_id: i64) -> Result<()> {
        self.conn.execute("DELETE FROM sessions WHERE user_id = ?1", params![user_id])?;
        Ok(())
    }

    pub fn cleanup_expired(&self) -> Result<usize> {
        let now = current_timestamp();
        let count = self.conn.execute(
            "DELETE FROM sessions WHERE expires_at < ?1",
            params![now],
        )?;
        Ok(count)
    }

    pub fn get_user_sessions(&self, user_id: i64) -> Result<Vec<Session>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, user_id, created_at, expires_at, last_accessed FROM sessions WHERE user_id = ?1 ORDER BY created_at DESC",
        )?;

        let sessions = stmt
            .query_map(params![user_id], Session::from_row)?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        Ok(sessions)
    }
}

#[cfg(test)]
mod tests {
    use crate::db::Database;

    #[test]
    fn test_session_crud() {
        let db = Database::open(":memory:").unwrap();
        db.initialize().unwrap();

        // Create user first
        db.users().create("testuser", "hash", None).unwrap();

        let store = db.sessions();

        // Create session
        let session = store.create("session123", 1, 3600).unwrap();
        assert_eq!(session.id, "session123");
        assert_eq!(session.user_id, 1);
        assert!(!session.is_expired());

        // Get session
        let found = store.get("session123").unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().user_id, 1);

        // Update access time
        let original_time = session.last_accessed;
        // Wait a bit to ensure timestamp changes
        std::thread::sleep(std::time::Duration::from_secs(1));
        store.update_access_time("session123").unwrap();
        let updated = store.get("session123").unwrap().unwrap();
        assert!(updated.last_accessed > original_time);

        // Get user sessions
        let sessions = store.get_user_sessions(1).unwrap();
        assert_eq!(sessions.len(), 1);

        // Delete session
        store.delete("session123").unwrap();
        assert!(store.get("session123").unwrap().is_none());
    }

    #[test]
    fn test_expired_session() {
        let db = Database::open(":memory:").unwrap();
        db.initialize().unwrap();

        db.users().create("testuser", "hash", None).unwrap();

        let store = db.sessions();

        // Create expired session (negative TTL to ensure it's expired)
        let session = store.create("expired", 1, -1).unwrap();
        assert!(session.is_expired());

        // Cleanup expired sessions
        let count = store.cleanup_expired().unwrap();
        assert_eq!(count, 1);
        assert!(store.get("expired").unwrap().is_none());
    }

    #[test]
    fn test_delete_by_user() {
        let db = Database::open(":memory:").unwrap();
        db.initialize().unwrap();

        db.users().create("testuser", "hash", None).unwrap();

        let store = db.sessions();

        // Create multiple sessions for same user
        store.create("session1", 1, 3600).unwrap();
        store.create("session2", 1, 3600).unwrap();

        let sessions = store.get_user_sessions(1).unwrap();
        assert_eq!(sessions.len(), 2);

        // Delete all user sessions
        store.delete_by_user(1).unwrap();
        let sessions = store.get_user_sessions(1).unwrap();
        assert_eq!(sessions.len(), 0);
    }
}
