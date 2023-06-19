mod handlers;
use std::ffi::OsString;

use axum::{routing::get, Router};
use handlers::{handler_dir, handler_markdown, handler_markdown_dangerous, handler_markdown_gdm};

pub fn create_routes(file_content: OsString, folder_path: std::path::PathBuf) -> Router<()> {
    Router::new()
            .route("/", get(handler_markdown_dangerous))
            .route("/safe", get(handler_markdown))
            .route("/gdm", get(handler_markdown_gdm))
            .fallback_service(handler_dir(&folder_path))
            .with_state(file_content)
}
