use futures_util::sink::SinkExt;
use std::io;
use the_construct::message::Message;
use tokio::{net::TcpStream, stream::StreamExt, time};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("0.0.0.0:4371").await?;
    let (mut read, mut write) = the_construct::message_streams(&mut stream);
    let now = std::time::Instant::now();
    loop {
        tokio::select! {
            Some(msg) = read.next() => {
                println!("Got: {:?}", msg?);
            }
            _ = time::delay_for(std::time::Duration::from_secs(rand::random::<u64>() % 10)) => {
                let msg = if rand::random() {
                    Message::play_request(now.elapsed())
                } else {
                    Message::pause(now.elapsed())
                };
                println!("Sending: {:?}", msg);
                write.send(msg).await?;
            }
        }
    }
}
