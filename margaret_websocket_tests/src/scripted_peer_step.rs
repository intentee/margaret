use tokio_tungstenite::tungstenite::Message;

pub enum ScriptedPeerStep {
    AwaitClientCancel,
    AwaitClientCredit,
    AwaitClientFrame,
    Send(Message),
}
