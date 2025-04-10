use anyhow::Result;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub struct PlaygroundConnection {}
// Function to handle a client connection
// need to pass in tcpStrem --> which will be an actal connection to the server, for now let's launch the server
impl PlaygroundConnection {
    fn handle_client(mut stream: TcpStream) {
        // Buffer to store incoming data
        let mut buffer = [0; 1024];

        // Read from the stream
        match stream.read(&mut buffer) {
            Ok(size) => {
                println!(
                    "Received data: {}",
                    String::from_utf8_lossy(&buffer[0..size])
                );

                // Echo back a response
                let response = "BAML Language Server Response";
                stream.write_all(response.as_bytes()).unwrap_or_default();
            }
            Err(e) => {
                println!("Error reading from client: {}", e);
            }
        }
    }

    // Public function to start the TCP server
    pub fn start_playground_server(port: u16) -> Result<()> {
        // Create a new listener binding to the provided port
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
        println!("BAML playground server listening on port {}", port);

        // Log to a file that we've started
        std::fs::write(
            "/tmp/baml-tcp-server.log",
            format!("TCP server started on port {}\n", port),
        )
        .unwrap_or_default();

        // Spawn a new thread to handle the TCP server
        thread::spawn(move || {
            // Accept connections in a loop
            // for stream in listener.incoming() {
            //     match stream {
            //         Ok(stream) => {
            //             // Spawn a new thread for each client
            //             thread::spawn(move || {
            //                 // handle_client(stream);
            //             });
            //         }
            //         Err(e) => {
            //             println!("Error accepting connection: {}", e);
            //         }
            //     }
            // }
        });

        Ok(())
    }
}
