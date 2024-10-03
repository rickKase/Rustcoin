use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            let bytes_read = socket.read(&mut buffer).await.unwrap();
            let message = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("Received: {}", message);

            socket.write_all(b"Hello from Rustcoin!").await.unwrap();
        });
    }
}
