use tokio_tungstenite::tungstenite::Message;

pub enum ScriptedPeerStep {
    AwaitClientCredit,
    AwaitClientFrame,
    Send(Message),
}
