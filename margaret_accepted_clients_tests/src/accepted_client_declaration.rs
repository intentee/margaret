use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::declares_accepted_client::DeclaresAcceptedClient;

pub struct AcceptedClientDeclaration {
    pub client: AcceptedClient,
}

impl DeclaresAcceptedClient for AcceptedClientDeclaration {
    fn accepted_client(&self) -> &AcceptedClient {
        &self.client
    }
}
