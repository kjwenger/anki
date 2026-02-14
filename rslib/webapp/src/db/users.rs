use anyhow::Result;
use rusqlite::{params, Connection, OptionalExtension, Row};
use serde::{Deserialize, Serialize};

use super::current_timestamp;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub email: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub is_active: bool,
    pub collection_path: Option<String>,
}

impl User {
    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password_hash: row.get(2)?,
            email: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
            is_active: row.get::<_, i64>(6)? != 0,
            collection_path: row.get(7)?,
        })
    }
}

pub struct UserStore<'a> {
    conn: &'a Connection,
}

impl<'a> UserStore<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn create(&self, username: &str, password_hash: &str, email: Option<&str>) -> Result<User> {
        let now = current_timestamp();
        self.conn.execute(
            "INSERT INTO users (username, password_hash, email, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![username, password_hash, email, now, now],
        )?;

        let id = self.conn.last_insert_rowid();
        self.get_by_id(id)?.ok_or_else(|| anyhow::anyhow!("Failed to retrieve created user"))
    }

    pub fn get_by_id(&self, id: i64) -> Result<Option<User>> {
        self.conn
            .query_row(
                "SELECT id, username, password_hash, email, created_at, updated_at, is_active, collection_path FROM users WHERE id = ?1",
                params![id],
                User::from_row,
            )
            .optional()
            .map_err(Into::into)
    }

    pub fn get_by_username(&self, username: &str) -> Result<Option<User>> {
        self.conn
            .query_row(
                "SELECT id, username, password_hash, email, created_at, updated_at, is_active, collection_path FROM users WHERE username = ?1",
                params![username],
                User::from_row,
            )
            .optional()
            .map_err(Into::into)
    }

    pub fn update_password(&self, user_id: i64, password_hash: &str) -> Result<()> {
        let now = current_timestamp();
        self.conn.execute(
            "UPDATE users SET password_hash = ?1, updated_at = ?2 WHERE id = ?3",
            params![password_hash, now, user_id],
        )?;
        Ok(())
    }

    pub fn update_collection_path(&self, user_id: i64, path: &str) -> Result<()> {
        let now = current_timestamp();
        self.conn.execute(
            "UPDATE users SET collection_path = ?1, updated_at = ?2 WHERE id = ?3",
            params![path, now, user_id],
        )?;
        Ok(())
    }

    pub fn set_active(&self, user_id: i64, is_active: bool) -> Result<()> {
        let now = current_timestamp();
        self.conn.execute(
            "UPDATE users SET is_active = ?1, updated_at = ?2 WHERE id = ?3",
            params![is_active as i64, now, user_id],
        )?;
        Ok(())
    }

    pub fn delete(&self, user_id: i64) -> Result<()> {
        self.conn.execute("DELETE FROM users WHERE id = ?1", params![user_id])?;
        Ok(())
    }

    pub fn list_all(&self) -> Result<Vec<User>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, username, password_hash, email, created_at, updated_at, is_active, collection_path FROM users ORDER BY username",
        )?;

        let users = stmt
            .query_map([], User::from_row)?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        Ok(users)
    }
}

#[cfg(test)]
mod tests {
    use crate::db::Database;

    #[test]
    fn test_user_crud() {
        let db = Database::open(":memory:").unwrap();
        db.initialize().unwrap();
        let store = db.users();

        // Create
        let user = store.create("testuser", "hash123", Some("test@example.com")).unwrap();
        assert_eq!(user.username, "testuser");
        assert_eq!(user.email.as_deref(), Some("test@example.com"));
        assert!(user.is_active);

        // Read by ID
        let found = store.get_by_id(user.id).unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().username, "testuser");

        // Read by username
        let found = store.get_by_username("testuser").unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, user.id);

        // Update password
        store.update_password(user.id, "newhash").unwrap();
        let updated = store.get_by_id(user.id).unwrap().unwrap();
        assert_eq!(updated.password_hash, "newhash");

        // Update collection path
        store.update_collection_path(user.id, "/path/to/collection").unwrap();
        let updated = store.get_by_id(user.id).unwrap().unwrap();
        assert_eq!(updated.collection_path.as_deref(), Some("/path/to/collection"));

        // Set inactive
        store.set_active(user.id, false).unwrap();
        let updated = store.get_by_id(user.id).unwrap().unwrap();
        assert!(!updated.is_active);

        // List all
        let users = store.list_all().unwrap();
        assert_eq!(users.len(), 1);

        // Delete
        store.delete(user.id).unwrap();
        assert!(store.get_by_id(user.id).unwrap().is_none());
    }

    #[test]
    fn test_unique_username() {
        let db = Database::open(":memory:").unwrap();
        db.initialize().unwrap();
        let store = db.users();

        store.create("testuser", "hash123", None).unwrap();
        let result = store.create("testuser", "hash456", None);
        assert!(result.is_err());
    }
}
