use axum::{extract::State, response::Html};
use std::{ffi::OsString, path::Path};
use tower_http::services::ServeDir;

pub async fn handler_markdown(State(file): State<OsString>) -> Html<String> {
    match std::fs::read_to_string(file) {
        Ok(content) => Html(markdown::to_html(&content)),
        Err(e) => Html(e.to_string()),
    }
}

pub async fn handler_markdown_gdm(State(file): State<OsString>) -> Html<String> {
    let content = match std::fs::read_to_string(file) {
        Ok(content) => content,
        Err(e) => return Html(e.to_string()),
    };

    match markdown::to_html_with_options(content.as_str(), &markdown::Options::gfm()) {
        Ok(content) => Html(content),
        Err(e) => Html(e),
    }
}
pub async fn handler_markdown_dangerous(State(file): State<OsString>) -> Html<String> {
    let content = match std::fs::read_to_string(file) {
        Ok(content) => content,
        Err(e) => return Html(e.to_string()),
    };
    match markdown::to_html_with_options(
        content.as_str(),
        &markdown::Options {
            compile: markdown::CompileOptions {
                allow_dangerous_html: true,
                allow_dangerous_protocol: true,
                ..markdown::CompileOptions::default()
            },
            ..markdown::Options::default()
        },
    ) {
        Ok(content) => Html(content),
        Err(e) => Html(e),
    }
}

pub fn handler_dir(path: &Path) -> ServeDir {
    ServeDir::new(path)
}
