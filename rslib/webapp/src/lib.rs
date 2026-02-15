// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

#![recursion_limit = "512"]

pub mod auth;
pub mod config;
pub mod db;
pub mod error;
pub mod openapi;
pub mod routes;
pub mod server;
pub mod session;
pub mod swagger_ui;

pub use auth::AuthState;
pub use auth::AuthUser;
pub use auth::JwtManager;
pub use config::WebAppConfig;
pub use db::Database;
pub use error::Result;
pub use error::WebAppError;
pub use routes::AuthRouteState;
pub use server::WebAppServer;
pub use session::BackendManager;
