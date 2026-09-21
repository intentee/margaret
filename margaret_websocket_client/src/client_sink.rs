use std::sync::Arc;

use futures_util::stream::SplitSink;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message;

use crate::client_stream::ClientStream;

pub(crate) type ClientSink = Arc<Mutex<SplitSink<ClientStream, Message>>>;
