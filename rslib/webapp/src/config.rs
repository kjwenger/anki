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
    /// Load configuration from file and environment variables
    /// Priority: ENV vars > config file > defaults
    pub fn from_env() -> anyhow::Result<Self> {
        // Start with default config
        let mut config = Self::default();
        
        // Try to load from config file (if exists)
        if let Ok(file_config) = Self::from_file("config/server.toml") {
            config = file_config;
        }
        
        // Override with environment variables (highest priority)
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

    /// Load configuration from a TOML file
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let config: WebAppConfig = toml::from_str(&contents)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_default_config() {
        let config = WebAppConfig::default();
        assert_eq!(config.host.to_string(), "127.0.0.1");
        assert_eq!(config.port, 8080);
        assert_eq!(config.data_dir, PathBuf::from("./data"));
        assert_eq!(config.jwt_secret, "change-this-secret-in-production");
        assert_eq!(config.session_timeout_hours, 24);
    }

    #[test]
    fn test_from_toml_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            r#"
host = "0.0.0.0"
port = 9000
data_dir = "/custom/path"
jwt_secret = "custom-secret"
session_timeout_hours = 48
"#
        )
        .unwrap();

        let config = WebAppConfig::from_file(temp_file.path().to_str().unwrap()).unwrap();
        assert_eq!(config.host.to_string(), "0.0.0.0");
        assert_eq!(config.port, 9000);
        assert_eq!(config.data_dir, PathBuf::from("/custom/path"));
        assert_eq!(config.jwt_secret, "custom-secret");
        assert_eq!(config.session_timeout_hours, 48);
    }

    #[test]
    fn test_partial_toml_config() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            r#"
port = 3000
jwt_secret = "my-secret"
"#
        )
        .unwrap();

        let config = WebAppConfig::from_file(temp_file.path().to_str().unwrap()).unwrap();
        // Specified values
        assert_eq!(config.port, 3000);
        assert_eq!(config.jwt_secret, "my-secret");
        // Default values for unspecified
        assert_eq!(config.host.to_string(), "127.0.0.1");
        assert_eq!(config.data_dir, PathBuf::from("./data"));
    }

    #[test]
    fn test_env_override() {
        std::env::set_var("ANKI_WEBAPP_PORT", "5555");
        std::env::set_var("ANKI_WEBAPP_JWT_SECRET", "env-secret");

        let config = WebAppConfig::from_env().unwrap();
        assert_eq!(config.port, 5555);
        assert_eq!(config.jwt_secret, "env-secret");

        std::env::remove_var("ANKI_WEBAPP_PORT");
        std::env::remove_var("ANKI_WEBAPP_JWT_SECRET");
    }
}

