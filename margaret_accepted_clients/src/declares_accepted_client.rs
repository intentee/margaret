use crate::accepted_client::AcceptedClient;

pub trait DeclaresAcceptedClient: Send + Sync {
    fn accepted_client(&self) -> &AcceptedClient;
}
