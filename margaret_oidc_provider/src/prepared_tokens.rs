use std::collections::BTreeSet;
use std::time::Duration;

use oauth2::AccessToken;
use oauth2::RefreshToken;
use oauth2::StandardTokenResponse;
use oauth2::basic::BasicTokenType;

use margaret_http::no_store::no_store;
use margaret_http::response::Response;
use margaret_identity_session::access_token_claims_signed::AccessTokenClaimsSigned;
use margaret_identity_session::access_token_lifetime_secs::ACCESS_TOKEN_LIFETIME_SECS;

use crate::provider_token_fields::ProviderTokenFields;
use crate::refresh_token_issue::RefreshTokenIssue;

pub(crate) struct PreparedTokens {
    pub(crate) access_token: AccessTokenClaimsSigned,
    pub(crate) fields: ProviderTokenFields,
    pub(crate) refresh: RefreshTokenIssue,
    pub(crate) scopes: BTreeSet<String>,
}

impl PreparedTokens {
    pub(crate) fn response(self) -> Response {
        let Self {
            access_token,
            fields,
            refresh,
            scopes,
        } = self;
        let mut response = StandardTokenResponse::new(
            AccessToken::new(access_token.signed_claims),
            BasicTokenType::Bearer,
            fields,
        );

        response.set_expires_in(Some(&Duration::from_secs(u64::from(
            ACCESS_TOKEN_LIFETIME_SECS,
        ))));
        response.set_refresh_token(match refresh {
            RefreshTokenIssue::Issued(token) => Some(RefreshToken::new(token)),
            RefreshTokenIssue::Withheld => None,
        });
        response.set_scopes(Some(scopes.into_iter().map(oauth2::Scope::new).collect()));

        no_store(Response::json(200, &response))
    }
}
