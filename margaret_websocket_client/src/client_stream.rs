use tokio_tungstenite::WebSocketStream;

use crate::client_io::ClientIo;

pub(crate) type ClientStream = WebSocketStream<Box<dyn ClientIo>>;
