// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

mod router;

use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::TcpListener;

use crate::auth::AuthState;
use crate::auth::JwtManager;
use crate::config::WebAppConfig;
use crate::db::Database;
use crate::error::Result;
use crate::session::BackendManager;

pub struct WebAppServer {
    config: WebAppConfig,
}

impl WebAppServer {
    pub fn new() -> anyhow::Result<Self> {
        let config = WebAppConfig::from_env()?;

        // Validate configuration
        if config.jwt_secret == "change-this-secret-in-production" {
            tracing::warn!(
                "Using default JWT secret - please set ANKI_WEBAPP_JWT_SECRET in production!"
            );
        }

        Ok(Self { config })
    }

    pub async fn run(self) -> Result<()> {
        let addr = SocketAddr::new(self.config.host, self.config.port);

        tracing::info!("Server configuration:");
        tracing::info!("  Host: {}", self.config.host);
        tracing::info!("  Port: {}", self.config.port);
        tracing::info!("  Data directory: {}", self.config.data_dir.display());

        // Ensure data directory exists
        std::fs::create_dir_all(&self.config.data_dir)
            .map_err(|e| anyhow::anyhow!("Failed to create data directory: {}", e))?;

        // Initialize database
        let db_path = self.config.data_dir.join("webapp.db");
        let database = Arc::new(Database::open(&db_path)?);
        database.initialize()?;
        tracing::info!("📦 Database initialized at {}", db_path.display());

        // Initialize JWT manager
        let jwt_manager = Arc::new(JwtManager::new(&self.config.jwt_secret));

        // Initialize backend manager
        let backend_manager = Arc::new(BackendManager::new(self.config.data_dir.clone()));
        tracing::info!("📋 Backend manager initialized");

        // Create auth state
        let auth_state = AuthState {
            database,
            jwt_manager,
            backend_manager,
        };

        // Build router
        let app = router::create_router(&self.config, auth_state);

        // Create listener
        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to bind to {}: {}", addr, e))?;

        tracing::info!("🚀 Anki Web App listening on http://{}", addr);
        tracing::info!("📚 Ready to serve!");

        // Run server
        axum::serve(listener, app)
            .await
            .map_err(|e| anyhow::anyhow!("Server error: {}", e))?;

        Ok(())
    }
}
