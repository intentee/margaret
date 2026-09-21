#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScriptedPeerClosing {
    Abruptly,
    Cleanly,
    StaysOpen,
}
