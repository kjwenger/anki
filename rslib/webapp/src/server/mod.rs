// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

mod router;

use crate::config::WebAppConfig;
use crate::error::Result;
use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

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
        
        // Build router
        let app = router::create_router();
        
        // Create listener
        let listener = TcpListener::bind(addr).await?;
        
        tracing::info!("🚀 Anki Web App listening on http://{}", addr);
        tracing::info!("📚 Ready to serve!");
        
        // Run server
        axum::serve(listener, app)
            .await
            .map_err(|e| anyhow::anyhow!("Server error: {}", e))?;
        
        Ok(())
    }
}
