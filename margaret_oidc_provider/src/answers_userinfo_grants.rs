use async_trait::async_trait;

use margaret_http::response::Response;

use crate::provider_error::ProviderError;
use crate::userinfo_grant::UserinfoGrant;

#[async_trait]
pub(crate) trait AnswersUserinfoGrants: Send + Sync {
    async fn answered(&self, grant: &UserinfoGrant) -> Result<Response, ProviderError>;
}
