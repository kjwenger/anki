use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use anki::collection::Collection;
use anki::collection::CollectionBuilder;
use anyhow::Result;

/// Manages per-user Anki Backend (Collection) instances
pub struct BackendManager {
    /// Map of user_id -> Collection instance
    backends: Arc<Mutex<HashMap<i64, Arc<Mutex<Collection>>>>>,
    /// Base directory for user collections
    data_dir: PathBuf,
}

impl BackendManager {
    pub fn new(data_dir: PathBuf) -> Self {
        Self {
            backends: Arc::new(Mutex::new(HashMap::new())),
            data_dir,
        }
    }

    /// Get or create a Collection instance for a user
    pub fn get_or_create_backend(
        &self,
        user_id: i64,
        username: &str,
    ) -> Result<Arc<Mutex<Collection>>> {
        let mut backends = self.backends.lock().unwrap();

        // Return existing backend if available
        if let Some(backend) = backends.get(&user_id) {
            return Ok(backend.clone());
        }

        // Create new backend
        let collection_path = self.get_collection_path(user_id, username);

        // Ensure user directory exists
        if let Some(parent) = collection_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        tracing::info!(
            "Opening collection for user {} at {:?}",
            username,
            collection_path
        );

        // Open or create collection
        let col = CollectionBuilder::new(collection_path).build()?;

        let backend = Arc::new(Mutex::new(col));
        backends.insert(user_id, backend.clone());

        Ok(backend)
    }

    /// Get existing backend for a user (without creating)
    pub fn get_backend(&self, user_id: i64) -> Option<Arc<Mutex<Collection>>> {
        let backends = self.backends.lock().unwrap();
        backends.get(&user_id).cloned()
    }

    /// Close and remove backend for a user
    pub fn close_backend(&self, user_id: i64) -> Result<()> {
        let mut backends = self.backends.lock().unwrap();

        if let Some(backend) = backends.remove(&user_id) {
            // Drop the backend to close the collection
            // The Collection's Drop implementation will handle cleanup
            drop(backend);
            tracing::info!("Closed collection for user {}", user_id);
        }

        Ok(())
    }

    /// Get the collection path for a user
    fn get_collection_path(&self, user_id: i64, username: &str) -> PathBuf {
        self.data_dir
            .join("users")
            .join(format!("user_{}", user_id))
            .join(format!("{}.anki2", username))
    }

    /// Get count of active backends
    pub fn active_backend_count(&self) -> usize {
        let backends = self.backends.lock().unwrap();
        backends.len()
    }

    /// Close all backends (for shutdown)
    pub fn close_all(&self) -> Result<()> {
        let mut backends = self.backends.lock().unwrap();
        let count = backends.len();

        backends.clear();

        if count > 0 {
            tracing::info!("Closed {} backend instances", count);
        }

        Ok(())
    }
}

impl Drop for BackendManager {
    fn drop(&mut self) {
        let _ = self.close_all();
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_backend_manager_lifecycle() {
        let temp_dir = TempDir::new().unwrap();
        let manager = BackendManager::new(temp_dir.path().to_path_buf());

        // Initially no backends
        assert_eq!(manager.active_backend_count(), 0);

        // Get backend for user 1
        let backend1 = manager.get_or_create_backend(1, "alice").unwrap();
        assert_eq!(manager.active_backend_count(), 1);

        // Getting same backend returns same instance
        let backend1_again = manager.get_or_create_backend(1, "alice").unwrap();
        assert_eq!(manager.active_backend_count(), 1);
        assert!(Arc::ptr_eq(&backend1, &backend1_again));

        // Get backend for user 2
        let _backend2 = manager.get_or_create_backend(2, "bob").unwrap();
        assert_eq!(manager.active_backend_count(), 2);

        // Close backend for user 1
        manager.close_backend(1).unwrap();
        assert_eq!(manager.active_backend_count(), 1);

        // Get returns None after close
        assert!(manager.get_backend(1).is_none());

        // Close all
        manager.close_all().unwrap();
        assert_eq!(manager.active_backend_count(), 0);
    }

    #[test]
    fn test_collection_path_generation() {
        let temp_dir = TempDir::new().unwrap();
        let manager = BackendManager::new(temp_dir.path().to_path_buf());

        let path = manager.get_collection_path(123, "testuser");

        assert!(path.to_string_lossy().contains("user_123"));
        assert!(path.to_string_lossy().ends_with("testuser.anki2"));
    }
}
