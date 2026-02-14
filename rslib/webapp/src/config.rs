// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAppConfig {
    #[serde(default = "default_host")]
    pub host: IpAddr,
    
    #[serde(default = "default_port")]
    pub port: u16,
    
    #[serde(default = "default_data_dir")]
    pub data_dir: PathBuf,
    
    #[serde(default = "default_jwt_secret")]
    pub jwt_secret: String,
    
    #[serde(default = "default_session_timeout_hours")]
    pub session_timeout_hours: u64,
}

fn default_host() -> IpAddr {
    "127.0.0.1".parse().unwrap()
}

fn default_port() -> u16 {
    8080
}

fn default_data_dir() -> PathBuf {
    PathBuf::from("./data")
}

fn default_jwt_secret() -> String {
    "change-this-secret-in-production".to_string()
}

fn default_session_timeout_hours() -> u64 {
    24
}

impl Default for WebAppConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            data_dir: default_data_dir(),
            jwt_secret: default_jwt_secret(),
            session_timeout_hours: default_session_timeout_hours(),
        }
    }
}

impl WebAppConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> anyhow::Result<Self> {
        let mut config = Self::default();
        
        // Override from environment variables
        if let Ok(host) = std::env::var("ANKI_WEBAPP_HOST") {
            config.host = host.parse()?;
        }
        
        if let Ok(port) = std::env::var("ANKI_WEBAPP_PORT") {
            config.port = port.parse()?;
        }
        
        if let Ok(data_dir) = std::env::var("ANKI_WEBAPP_DATA_DIR") {
            config.data_dir = PathBuf::from(data_dir);
        }
        
        if let Ok(jwt_secret) = std::env::var("ANKI_WEBAPP_JWT_SECRET") {
            config.jwt_secret = jwt_secret;
        }
        
        if let Ok(timeout) = std::env::var("ANKI_WEBAPP_SESSION_TIMEOUT_HOURS") {
            config.session_timeout_hours = timeout.parse()?;
        }
        
        Ok(config)
    }
}
