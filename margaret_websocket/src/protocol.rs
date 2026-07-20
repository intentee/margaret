use async_trait::async_trait;
use serde_json::Value;

use crate::activity_spawner::ActivitySpawner;
use crate::emit_core::EmitCore;
use crate::request_id::RequestId;

#[async_trait]
pub trait Protocol: Send + Sync + 'static {
    type Internal: Send + 'static;
    type State: Send;

    fn initial_state(&self) -> Self::State;

    fn is_terminal(&self, state: &Self::State) -> bool;

    async fn dispatch(
        &self,
        state: Self::State,
        method: &str,
        params: Option<&Value>,
        request_id: Option<RequestId>,
        emit: &EmitCore,
        spawner: &ActivitySpawner<Self::Internal>,
    ) -> Self::State;

    async fn dispatch_internal(
        &self,
        state: Self::State,
        event: Self::Internal,
        emit: &EmitCore,
        spawner: &ActivitySpawner<Self::Internal>,
    ) -> Self::State;
}
