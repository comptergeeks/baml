use axum::{routing::get_service, Router};
use std::net::SocketAddr;
use std::path::{absolute, PathBuf};
use tower_http::services::{ServeDir, ServeFile};

pub async fn serve_playground(dist_path: PathBuf, port: u16) {
    // Log the start of the function
    std::fs::write(
        "/tmp/baml-lsp-debug.log",
        format!("Serving the playground\n"),
    )
    .unwrap_or_default();

    // Log the path we're serving from
    std::fs::write(
        "/tmp/baml-lsp-debug.log",
        format!("Serving the playground from: {}\n", dist_path.display()),
    )
    .unwrap_or_default();

    std::fs::write("/tmp/baml-lsp-debug.log", format!("trying port {}\n", port))
        .unwrap_or_default();
    // Create the fallback file path (index.html in the same directory)
    let fallback_file = dist_path.join("index.html");

    let app = Router::new()
        .fallback_service(ServeDir::new(dist_path).fallback(ServeFile::new(fallback_file)));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    std::fs::write(
        "/tmp/baml-lsp-debug.log",
        format!("Playground available at http://{}", addr),
    )
    .unwrap_or_default();

    // Start the server with detailed error handling
    match axum_server::bind(addr).serve(app.into_make_service()).await {
        Ok(_) => {
            println!("Playground server finished successfully");
            std::fs::write(
                "/tmp/baml-lsp-debug.log",
                "Playground server finished successfully\n",
            )
            .unwrap_or_default();
        }
        Err(e) => {
            let error_msg = format!("Failed to start playground server: {:?}\n", e);
            eprintln!("{}", error_msg);
            std::fs::write("/tmp/baml-lsp-debug.log", error_msg).unwrap_or_default();
        }
    }
}
