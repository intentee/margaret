use serde::Serialize;

pub trait WebsocketOutbound: Serialize {
    fn wire_method(&self) -> &'static str;
}
