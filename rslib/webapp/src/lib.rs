// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

pub mod config;
pub mod error;
pub mod server;

pub use config::WebAppConfig;
pub use error::{Result, WebAppError};
pub use server::WebAppServer;
