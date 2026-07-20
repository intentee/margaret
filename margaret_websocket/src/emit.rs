use std::marker::PhantomData;

use serde_json::to_value;

use crate::emit_core::EmitCore;
use crate::json_rpc_error_frame::JsonRpcErrorFrame;
use crate::outbound_frame::OutboundFrame;
use crate::websocket_outbound::WebsocketOutbound;

pub struct Emit<Outbound> {
    core: EmitCore,
    outbound: PhantomData<Outbound>,
}

impl<Outbound> Emit<Outbound>
where
    Outbound: WebsocketOutbound,
{
    #[must_use]
    pub fn new(core: EmitCore) -> Self {
        Self {
            core,
            outbound: PhantomData,
        }
    }

    pub async fn push(&self, message: impl Into<Outbound>) {
        let outbound = message.into();
        let method = outbound.wire_method().to_owned();

        match to_value(&outbound) {
            Ok(params) => self.core.send(OutboundFrame::Notify { method, params }).await,
            Err(error) => {
                eprintln!(
                    "margaret_websocket: an outbound message could not be serialized: {error}"
                );

                self.core
                    .send(OutboundFrame::Error(JsonRpcErrorFrame::internal_error(None)))
                    .await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::Serialize;
    use serde::Serializer;
    use serde::ser::Error;
    use serde_json::Value;
    use tokio::sync::mpsc::channel;
    use tokio_util::sync::CancellationToken;

    use super::Emit;
    use crate::emit_core::EmitCore;
    use crate::outbound_frame::OutboundFrame;
    use crate::websocket_outbound::WebsocketOutbound;

    #[derive(Serialize)]
    struct Reply {
        text: String,
    }

    impl WebsocketOutbound for Reply {
        fn wire_method(&self) -> &'static str {
            "reply"
        }
    }

    struct Unserializable;

    impl Serialize for Unserializable {
        fn serialize<Target: Serializer>(
            &self,
            _serializer: Target,
        ) -> Result<Target::Ok, Target::Error> {
            Err(Target::Error::custom("this outbound message cannot be serialized"))
        }
    }

    impl WebsocketOutbound for Unserializable {
        fn wire_method(&self) -> &'static str {
            "unserializable"
        }
    }

    fn frame_value(frame: OutboundFrame) -> Value {
        let text = frame
            .into_message()
            .into_text()
            .expect("the outbound frame is a text message");

        serde_json::from_str(text.as_str()).expect("the text frame is valid json")
    }

    #[tokio::test]
    async fn push_sends_a_notify_frame_labelled_with_the_wire_method() {
        let (sender, mut receiver) = channel(1);
        let emit: Emit<Reply> = Emit::new(EmitCore::new(CancellationToken::new(), sender));

        emit.push(Reply {
            text: "hi".to_owned(),
        })
        .await;

        let value = frame_value(receiver.recv().await.expect("a frame is emitted"));

        assert_eq!(value["method"], "reply");
        assert_eq!(value["params"]["text"], "hi");
    }

    #[tokio::test]
    async fn push_reports_an_internal_error_when_serialization_fails() {
        let (sender, mut receiver) = channel(1);
        let emit: Emit<Unserializable> = Emit::new(EmitCore::new(CancellationToken::new(), sender));

        emit.push(Unserializable).await;

        let value = frame_value(receiver.recv().await.expect("a frame is emitted"));

        assert_eq!(value["error"]["code"], -32603);
    }
}
