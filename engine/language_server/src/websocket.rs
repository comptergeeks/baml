// websocket.rs
use axum::{routing::get, Router};
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
/// A server that creates a Socket.IO WebSocket endpoint.
pub struct WebSocketServer;

impl WebSocketServer {
    /// Creates and runs a Socket.IO server on `127.0.0.1:6969`.
    ///
    /// This function:
    /// - Sets up logging (if not already configured).
    /// - Creates the Socket.IO layer using socketioxide.
    /// - Registers a connection handler on the default namespace (`"/"`) that listens for a `"message"` event.
    /// - Starts an Axum-based HTTP server, which handles both HTTP requests (e.g. for health-checks)
    ///   and upgrades WebSocket connections according to the Socket.IO protocol.
    ///
    /// When a client sends a `"message"` event, the handler logs the inner message (a String)
    /// and echoes it back to the client.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     // Launch the WebSocket server.
    ///     WebSocketServer::create_websocket().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_websocket() -> Result<(), Box<dyn std::error::Error>> {
        // Initialize logging via tracing (ignore error if already set).
        let _ = tracing::subscriber::set_global_default(FmtSubscriber::default());

        // Create the Socket.IO layer and obtain the io handle.
        let (_socket_layer, io) = SocketIo::new_layer();

        // Register a connection handler on the default namespace ("/").
        // The closure signature uses explicit types for clarity.
        io.ns("/", |socket: SocketRef| {
            info!("Socket.IO client connected: {}", socket.id);

            // Register a handler for the "message" event.
            socket.on("message", |socket: SocketRef, Data(data): Data<String>| {
                // Instead of logging ?data (which requires Debug on Data<String>),
                // we log the inner String directly.
                info!("Received message event: {}", data);
                // Echo the received message back to the client.
                socket.emit("message", &data).ok();
            });
        });

        std::fs::write(
            "/tmp/baml-lsp-debug.log",
            format!("trying to launch webserver\n"),
        )
        .unwrap_or_default();

        let app = Router::new().route("/", get(|| async { "Hello, world!" }));
        let addr = SocketAddr::from(([127, 0, 0, 1], 6969));
        // Only use one server approach - the Axum server is sufficient
        axum_server::bind(addr)
            .serve(app.into_make_service())
            .await?; // Use ? to propagate the error instead of unwrap()
                     // now let's launch the react app for now

        // Launch the playground server
        Ok(())
    }
}
