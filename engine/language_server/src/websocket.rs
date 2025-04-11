use axum::{routing::get, Router};
use socketioxide::{extract::SocketRef, SocketIo};
use std::{net::SocketAddr, path::PathBuf, sync::Mutex};
use tracing::info;

// A simple WebSocket server
pub struct WebSocketServer;

// Global Socket.IO instance
static IO: std::sync::OnceLock<Mutex<Option<SocketIo>>> = std::sync::OnceLock::new();

impl WebSocketServer {
    pub async fn create_websocket() -> Result<(), Box<dyn std::error::Error>> {
        // Create the Socket.IO layer
        let (socket_layer, io) = SocketIo::new_layer();

        // Store IO instance
        IO.get_or_init(|| Mutex::new(Some(io.clone())));

        // Handle connections
        io.ns("/", |socket: SocketRef| {
            info!("Client connected: {}", socket.id);
        });

        // Create HTTP server
        let app = Router::new()
            .route("/", get(|| async { "BAML WebSocket Server" }))
            .layer(socket_layer);

        let addr = SocketAddr::from(([127, 0, 0, 1], 6969));
        axum_server::bind(addr)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }

    // Send document path to clients
    pub async fn send_path(path: &PathBuf) -> Result<(), String> {
        let path_str = path.to_string_lossy();
        info!("Attempting to send path: {}", path_str);

        // Get IO instance
        let io_lock = IO.get().ok_or("IO not initialized")?;
        let io_guard = io_lock.lock().map_err(|_| "Failed to lock IO")?;

        if let Some(io) = &*io_guard {
            // Either use ?, or explicitly handle the result with match or if let
            match io.emit("document_saved", &path_str).await {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Failed to emit event: {}", e)),
            }
        } else {
            Err("SocketIo not initialized".to_string())
        }
    }
}
