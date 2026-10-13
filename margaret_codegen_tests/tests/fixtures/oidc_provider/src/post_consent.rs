use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::serves_oidc_endpoint;
use margaret::framework::oidc_provider::oidc_endpoint::OidcEndpoint;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::consent_view::ConsentView;

#[responds_to_http(
    max_body_bytes = 4096,
    method = RouteMethod::Post,
    path = "/consent",
    server = "public"
)]
#[serves_oidc_endpoint(OidcEndpoint::Consent(view = ConsentView))]
pub struct PostConsent;
