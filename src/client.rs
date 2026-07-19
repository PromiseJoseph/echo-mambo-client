use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::ECHOMAMBO_SERVER_ADDR;

pub async fn call_echo_mambo(message: &str) -> String {
    let server_addr_as_arg = std::env::args().nth(2);

    println!("Connecting to EchoMambo...");

    // Connect to the EchoMambo server
    let mut stream =
        connect_to_server(&server_addr_as_arg.unwrap_or(ECHOMAMBO_SERVER_ADDR.to_string())).await;

    println!(
        "Connection established with {}!",
        stream.peer_addr().unwrap()
    );

    let incoming_message = &message.to_string();

    stream
        .write_all(incoming_message.as_bytes())
        .await
        .expect("Failed to send message to server");

    println!("Sent message to server: \"{}\"", message);

    let mut buffer = [0; 1024];
    let bytes_read = match stream.read(&mut buffer).await {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Error: {}", e);
            return String::from("Failed to read response from server.");
        }
    };

    if bytes_read == 0 {
        return String::from("Server closed the connection unexpectedly.");
    }

    let response = String::from_utf8_lossy(&buffer[..bytes_read]);
    return response.into_owned();
}

//=== handles connection to server ===//
async fn connect_to_server(server_addr: &str) -> TcpStream {
    if let Ok(stream) = TcpStream::connect(server_addr).await {
        stream
    } else {
        panic!("Failed to connect to server at {}", server_addr);
    }
}
