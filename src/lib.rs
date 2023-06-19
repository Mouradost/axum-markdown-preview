use axum::Router;
use router::create_routes;
use colored::*;
use std::net::SocketAddr;

mod router;

pub async fn run_server(file: std::path::PathBuf) -> anyhow::Result<()> {

    let app: Router = create_routes(file.clone().into_os_string(), file.parent().unwrap().to_path_buf());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("{}", format!("Preview on http://{}", addr).green());
    println!("{} {}", format!("Use http://{}/gdm for GitHub like Preview", addr).italic(), "Markdown".italic().blue());
    println!("{}", format!("Use http://{}/safe to not execute user html tags like Preview", addr).italic());

    open::that(format!("http://{}", addr))?;

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
