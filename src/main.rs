use tokio::net::{TcpStream, TcpListener};
use tokio::io::{AsyncReadExt,AsyncWriteExt,};
mod client;
use client::call_echo_mambo;

const ECHOMAMBO_CLIENT_ADDR: &str = "127.0.0.1:7778";
const ECHOMAMBO_SERVER_ADDR: &str = "127.0.0.1:7777";
 

const BUFFER_SIZE: usize = 1024;
    
#[tokio::main]
async fn main() {

println!("Starting EchoMamboClient on {}", ECHOMAMBO_CLIENT_ADDR);

let listener: TcpListener  = TcpListener::bind(ECHOMAMBO_CLIENT_ADDR).await.expect("Failed to bind to address");

println!("EchoMamboClient is listening for incoming on {}", ECHOMAMBO_CLIENT_ADDR);

//loop handles multiple incoming connections from user clients
loop{
    let(stream, _) = listener.accept().await.expect("Failed to accept incoming connection");
    println!("Accepted connection from {}", stream.peer_addr().unwrap());
    tokio::spawn(async move {
        handle_client(stream).await;
    });
 }
}

async fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; BUFFER_SIZE];
    //inner loop handles multiple messages from a single user client
    loop {
          let data_byte  = match stream.read(&mut buffer).await {
                Ok(0) => {
                    println!("Client closed the connection.");
                    break;
                }
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                    break;
                }
            };   
  
    let message = String::from_utf8_lossy(&buffer[..data_byte]).to_string();
    println!("Received message: {} : length {}", message, data_byte );
    
    let echomambo_message = call_echo_mambo(&message).await;

    stream.write_all(echomambo_message.as_bytes()).await.unwrap(); // Send the response back to the user client
 
    println!("EchoMambo responded:  {}", echomambo_message);

}   

}


