/*https://www.youtube.com/watch?v=Iapc-qGTEBQ

31:41 */

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();
        
        tokio::spawn(async move {
            let (read, mut writer) = socket.split();
            let mut reader = BufReader::new(read);

            let mut line = String::new();

            loop {
                let bytes_read: usize = reader.read_line(&mut line).await.unwrap();

                if bytes_read == 0 { break; }

                writer.write_all(&line.as_bytes()).await.unwrap();
                line.clear();
            }
        });
    }
}
