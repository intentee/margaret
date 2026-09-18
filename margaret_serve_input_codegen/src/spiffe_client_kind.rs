#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SpiffeClientKind {
    Http,
    WebSocket,
}
