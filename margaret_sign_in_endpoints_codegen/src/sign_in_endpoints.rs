use margaret_attributes::framework_vocabulary::FrameworkVocabulary;

use crate::sign_in_endpoint_variant::SignInEndpointVariant;

fn sign_in_endpoint_name(variant: SignInEndpointVariant) -> &'static str {
    match variant {
        SignInEndpointVariant::Callback => "Callback",
        SignInEndpointVariant::Start => "Start",
    }
}

pub(crate) const SIGN_IN_ENDPOINTS: FrameworkVocabulary<SignInEndpointVariant> =
    FrameworkVocabulary {
        enum_path: &[
            "margaret",
            "framework",
            "oidc_sign_in",
            "sign_in_endpoint",
            "SignInEndpoint",
        ],
        name: sign_in_endpoint_name,
        variants: &[
            SignInEndpointVariant::Callback,
            SignInEndpointVariant::Start,
        ],
    };
