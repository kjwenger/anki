// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use std::sync::Arc;
use tokio::net::TcpListener;
use tempfile::TempDir;
use reqwest::Client;

use anki_webapp::auth::{AuthState, JwtManager, Claims};
use anki_webapp::config::WebAppConfig;
use anki_webapp::db::Database;
use anki_webapp::session::BackendManager;
use anki_webapp::server::router::create_router;

pub struct TestContext {
    pub client: Client,
    pub base_url: String,
    #[allow(dead_code)]
    pub config: WebAppConfig,
    #[allow(dead_code)]
    pub database: Arc<Database>,
    #[allow(dead_code)]
    pub jwt_manager: Arc<JwtManager>,
    #[allow(dead_code)]
    pub backend_manager: Arc<BackendManager>,
    _temp_dir: TempDir,
}

impl TestContext {
    pub async fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let data_dir = temp_dir.path().to_path_buf();
        
        let mut config = WebAppConfig {
            host: "127.0.0.1".parse().unwrap(),
            port: 0,
            data_dir: data_dir.clone(),
            jwt_secret: "test-secret-must-be-long-enough-for-hs256-at-least-32-chars".to_string(),
            session_timeout_hours: 24,
        };

        let db_path = data_dir.join("webapp.db");
        let database = Arc::new(Database::open(&db_path).expect("Failed to open test database"));
        database.initialize().expect("Failed to initialize test database");

        let jwt_manager = Arc::new(JwtManager::new(&config.jwt_secret));
        let backend_manager = Arc::new(BackendManager::new(data_dir.clone()));

        let auth_state = AuthState {
            database: database.clone(),
            jwt_manager: jwt_manager.clone(),
            backend_manager: backend_manager.clone(),
        };

        let app_router = create_router(&config, auth_state);
        
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("Failed to bind to random port");
        let addr = listener.local_addr().expect("Failed to get local addr");
        config.port = addr.port();
        let base_url = format!("http://{}", addr);

        tokio::spawn(async move {
            axum::serve(listener, app_router).await.expect("Server failed");
        });

        let client = Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Failed to build reqwest client");

        Self {
            client,
            base_url,
            config,
            database,
            jwt_manager,
            backend_manager,
            _temp_dir: temp_dir,
        }
    }

    #[allow(dead_code)]
    pub fn generate_token(&self, user_id: i64, username: &str) -> String {
        let claims = Claims::new(user_id, username.to_string(), "test-session".to_string(), self.config.session_timeout_hours as i64);
        self.jwt_manager.generate_token(&claims)
            .expect("Failed to create test token")
    }
}
