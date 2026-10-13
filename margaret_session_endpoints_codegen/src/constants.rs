use margaret_attributes::framework_vocabulary::FrameworkVocabulary;

use crate::session_endpoint_variant::SessionEndpointVariant;

fn session_endpoint_name(variant: SessionEndpointVariant) -> &'static str {
    match variant {
        SessionEndpointVariant::Refresh => "Refresh",
        SessionEndpointVariant::SignOut => "SignOut",
    }
}

pub(crate) const SESSION_ENDPOINTS: FrameworkVocabulary<SessionEndpointVariant> =
    FrameworkVocabulary {
        enum_path: &[
            "margaret",
            "framework",
            "sessions",
            "session_endpoint",
            "SessionEndpoint",
        ],
        name: session_endpoint_name,
        variants: &[
            SessionEndpointVariant::Refresh,
            SessionEndpointVariant::SignOut,
        ],
    };
