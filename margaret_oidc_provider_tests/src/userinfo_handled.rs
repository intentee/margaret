use std::sync::Arc;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::head_handler::HeadHandler;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_oidc_provider::provides_userinfo_claims::ProvidesUserinfoClaims;
use margaret_oidc_provider::userinfo_endpoint::UserinfoEndpoint;

use crate::request_authorized_by::request_authorized_by;
use crate::unserved_provider::UnservedProvider;
use crate::userinfo_access_token::userinfo_access_token;

/// # Errors
///
/// Returns the `HandlerError` the userinfo endpoint reports for the request.
pub async fn userinfo_handled<TProvider: ProvidesUserinfoClaims>(
    subject: &str,
    claims: TProvider,
) -> Result<ResponseContinuation, HandlerError> {
    let provider = UnservedProvider::create();
    let access_token = userinfo_access_token(&provider, subject);

    UserinfoEndpoint::create(provider.secret_store, provider.issuance, Arc::new(claims))
        .handle(&request_authorized_by(&format!("Bearer {access_token}")))
        .await
}
