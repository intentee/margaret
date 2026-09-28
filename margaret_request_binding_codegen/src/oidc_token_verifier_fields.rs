use crate::bound_parameter::BoundParameter;
use crate::oidc_token_verifier_field::OidcTokenVerifierField;
use crate::request_binding::RequestBinding;

#[must_use]
pub fn oidc_token_verifier_fields(parameters: &[BoundParameter]) -> Vec<OidcTokenVerifierField> {
    parameters
        .iter()
        .filter_map(|parameter| match &parameter.binding {
            RequestBinding::OidcToken { verifier, .. } => Some(verifier.clone()),
            _ => None,
        })
        .collect()
}
