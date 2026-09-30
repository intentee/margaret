use std::sync::Arc;

use crate::declares_accepted_client::DeclaresAcceptedClient;
use crate::registered_secret::RegisteredSecret;

pub(crate) struct RegisteredClient {
    pub(crate) declaration: Arc<dyn DeclaresAcceptedClient>,
    pub(crate) secret: RegisteredSecret,
}
