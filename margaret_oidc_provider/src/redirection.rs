use url::Url;

use margaret_http::response::Response;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

use crate::authorization_error::AuthorizationError;
use crate::frame_denied::frame_denied;

pub(crate) struct Redirection<'issuer> {
    pub(crate) issuer: &'issuer IssuerIdentifier,
    pub(crate) redirect_uri: Url,
    pub(crate) state: Option<String>,
}

impl Redirection<'_> {
    pub(crate) fn code(self, code: &str) -> Response {
        self.redirected("code", code)
    }

    pub(crate) fn error(self, error: AuthorizationError) -> Response {
        self.redirected("error", error.wire_name())
    }

    fn redirected(self, name: &str, value: &str) -> Response {
        let Self {
            issuer,
            mut redirect_uri,
            state,
        } = self;

        {
            let mut query = redirect_uri.query_pairs_mut();

            query.append_pair(name, value);
            query.append_pair("iss", issuer.as_str());

            if let Some(state) = &state {
                query.append_pair("state", state);
            }
        }

        frame_denied(Response::text(303, "").header("location", redirect_uri.as_str()))
    }
}
