use margaret_http::response::Response;

use crate::consent_request::ConsentRequest;

pub enum AuthorizationOutcome {
    AuthenticationRequired { return_to: String },
    ConsentRequired(ConsentRequest),
    Redirected(Response),
    Rejected(Response),
}
