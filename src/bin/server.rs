use futures_util::sink::SinkExt;
use std::io;
use the_construct::message::Message;
use tokio::{
    net::{TcpListener, TcpStream},
    stream::StreamExt,
    sync::broadcast,
    task,
};

#[derive(Debug)]
enum Error {
    Io(io::Error),
    RChannel(broadcast::RecvError),
    SChannel(broadcast::SendError<Message>),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<broadcast::RecvError> for Error {
    fn from(e: broadcast::RecvError) -> Self {
        Self::RChannel(e)
    }
}

impl From<broadcast::SendError<Message>> for Error {
    fn from(e: broadcast::SendError<Message>) -> Self {
        Self::SChannel(e)
    }
}

async fn handle_connection(
    mut stream: TcpStream,
    sx: broadcast::Sender<Message>,
    mut incoming: broadcast::Receiver<Message>,
    name: &str,
) -> Result<(), Error> {
    let (mut read, mut write) = the_construct::message_streams(&mut stream);
    loop {
        tokio::select! {
            msg = incoming.recv() => {
                eprintln!("{} => got from bc {:?}", name, msg);
                write.send(msg?).await?;
            }
            msg = read.try_next() => {
                eprintln!("{} => got from socket {:?}", name, msg);
                match msg? {
                    Some(Message::PlayRequest { timestamp, .. }) => {
                        sx.send(Message::play_command(timestamp))?;
                    }
                    Some(Message::PlayCommand { .. }) => {
                        eprintln!("{} => Only server can send play command", name);
                    }
                    Some(msg) => {
                        eprintln!("{} => broadcasting {:?}", name, msg);
                        sx.send(msg)?;
                    }
                    None => return Ok(())
                }
            }
        };
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut listener = TcpListener::bind("0.0.0.0:4371").await?;
    let mut incoming = listener.incoming();
    let (broadcast, _r) = broadcast::channel(4096);
    let mut c = 0;
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        let sub = broadcast.subscribe();
        let broadcast = broadcast.clone();
        task::spawn(async move {
            let name = format!("Task {}", c);
            eprintln!("Got a connection. Creating {}", c);
            eprintln!(
                "{} => terminating: {:?}",
                name,
                handle_connection(stream, broadcast, sub, &name).await
            )
        });
        c += 1;
    }
    Ok(())
}
