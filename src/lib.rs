pub mod message;
use futures_util::sink::Sink;
use message::Message;
use tokio::{net::TcpStream, stream::Stream};
use tokio_serde::formats;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

pub fn message_streams(
    socket: &mut TcpStream,
) -> (
    impl Stream<Item = std::io::Result<Message>> + '_,
    impl Sink<Message, Error = std::io::Error> + '_,
) {
    let (read, write) = socket.split();
    (
        tokio_serde::SymmetricallyFramed::new(
            FramedRead::new(read, LengthDelimitedCodec::new()),
            formats::SymmetricalBincode::<Message>::default(),
        ),
        tokio_serde::SymmetricallyFramed::new(
            FramedWrite::new(write, LengthDelimitedCodec::new()),
            formats::SymmetricalBincode::<Message>::default(),
        ),
    )
}
