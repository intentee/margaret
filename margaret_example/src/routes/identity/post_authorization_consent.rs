use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::serves_oidc_endpoint;
use margaret::framework::oidc_provider::oidc_endpoint::OidcEndpoint;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::views::consent_view::ConsentView;

#[responds_to_http(
    max_body_bytes = 8_388_608,
    method = RouteMethod::Post,
    name = "post_authorization_consent",
    path = "/authorize/consent",
    server = "identity"
)]
#[serves_oidc_endpoint(OidcEndpoint::Consent(view = ConsentView))]
pub struct PostAuthorizationConsent;
