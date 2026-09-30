use std::time::Duration;

use oauth2::AccessToken;
use oauth2::RefreshToken;
use oauth2::StandardTokenResponse;
use oauth2::basic::BasicTokenType;

use margaret_http::response::Response;
use margaret_identity_session::access_token_claims_signed::AccessTokenClaimsSigned;
use margaret_identity_session::access_token_lifetime_secs::ACCESS_TOKEN_LIFETIME_SECS;
use margaret_oauth_vocabulary::scope_list::ScopeList;

use crate::no_store::no_store;
use crate::provider_token_fields::ProviderTokenFields;

pub(crate) fn token_response(
    access_token: AccessTokenClaimsSigned,
    scope: &ScopeList,
    refresh_token: Option<String>,
    fields: ProviderTokenFields,
) -> Response {
    let mut response = StandardTokenResponse::new(
        AccessToken::new(access_token.signed_claims),
        BasicTokenType::Bearer,
        fields,
    );

    response.set_expires_in(Some(&Duration::from_secs(u64::from(
        ACCESS_TOKEN_LIFETIME_SECS,
    ))));
    response.set_refresh_token(refresh_token.map(RefreshToken::new));
    response.set_scopes(Some(
        scope
            .scopes
            .iter()
            .map(|scope| oauth2::Scope::new(scope.as_str().to_string()))
            .collect(),
    ));

    no_store(Response::json(200, &response))
}
