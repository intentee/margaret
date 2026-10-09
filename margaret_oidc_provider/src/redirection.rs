use url::Url;
use url::UrlQuery;
use url::form_urlencoded::Serializer;

use margaret_http::redirect::Redirect;
use margaret_http::response::Response;

use crate::authorization_error::AuthorizationError;

pub(crate) struct Redirection {
    pub(crate) issuer: &'static str,
    pub(crate) redirect_uri: Url,
    pub(crate) state: Option<String>,
}

impl Redirection {
    pub(crate) fn code(self, code: &str) -> Response {
        self.redirected(|query| {
            query.append_pair("code", code);
        })
    }

    pub(crate) fn error(self, error: &AuthorizationError) -> Response {
        self.redirected(|query| {
            query.append_pair("error", error.wire_name());
            query.append_pair("error_description", error.description());
        })
    }

    fn redirected(self, outcome: impl FnOnce(&mut Serializer<UrlQuery<'_>>)) -> Response {
        let Self {
            issuer,
            mut redirect_uri,
            state,
        } = self;

        {
            let mut query = redirect_uri.query_pairs_mut();

            outcome(&mut query);
            query.append_pair("iss", issuer);

            if let Some(state) = &state {
                query.append_pair("state", state);
            }
        }

        Redirect::see_other(redirect_uri.into()).into_response()
    }
}
