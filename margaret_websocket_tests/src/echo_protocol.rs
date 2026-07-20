use async_trait::async_trait;
use serde_json::Value;

use margaret_websocket::activity_spawner::ActivitySpawner;
use margaret_websocket::emit_core::EmitCore;
use margaret_websocket::envelope::Envelope;
use margaret_websocket::outbound_frame::OutboundFrame;
use margaret_websocket::protocol::Protocol;
use margaret_websocket::request_id::RequestId;

use crate::echo_state::EchoState;
use crate::echo_tick::EchoTick;

pub struct EchoProtocol;

#[async_trait]
impl Protocol for EchoProtocol {
    type Internal = EchoTick;
    type State = EchoState;

    fn initial_state(&self) -> EchoState {
        EchoState::Ready
    }

    fn is_terminal(&self, state: &EchoState) -> bool {
        matches!(state, EchoState::Ended)
    }

    async fn dispatch(
        &self,
        _state: EchoState,
        method: &str,
        _params: Option<&Value>,
        request_id: Option<RequestId>,
        emit: &EmitCore,
        spawner: &ActivitySpawner<EchoTick>,
    ) -> EchoState {
        match request_id {
            Some(id) => {
                let envelope = Envelope::new(id, Value::from(method.to_owned()));

                emit.send(OutboundFrame::Result {
                    id: envelope.id,
                    result: envelope.message,
                })
                .await;

                let internal_sender = spawner.internal_sender();

                spawner.spawn(async move {
                    let _ = internal_sender.send(EchoTick).await;
                });

                EchoState::Ready
            }
            None => EchoState::Ended,
        }
    }

    async fn dispatch_internal(
        &self,
        state: EchoState,
        _event: EchoTick,
        emit: &EmitCore,
        _spawner: &ActivitySpawner<EchoTick>,
    ) -> EchoState {
        emit.send(OutboundFrame::Notify {
            method: "tick".to_owned(),
            params: Value::Null,
        })
        .await;

        state
    }
}
