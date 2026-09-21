use tokio_tungstenite::tungstenite::Message;

pub enum ScriptedPeerStep {
    AwaitClientFrame,
    Send(Message),
}
