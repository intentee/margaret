use margaret_attributes::framework_vocabulary::FrameworkVocabulary;

use crate::session_cookies_variant::SessionCookiesVariant;

fn session_cookies_name(variant: SessionCookiesVariant) -> &'static str {
    match variant {
        SessionCookiesVariant::HostOnly => "HostOnly",
        SessionCookiesVariant::SharedWithDomain => "SharedWithDomain",
    }
}

pub(crate) const SESSION_COOKIES: FrameworkVocabulary<SessionCookiesVariant> =
    FrameworkVocabulary {
        enum_path: &[
            "margaret",
            "framework",
            "sessions",
            "session_cookies",
            "SessionCookies",
        ],
        name: session_cookies_name,
        variants: &[
            SessionCookiesVariant::HostOnly,
            SessionCookiesVariant::SharedWithDomain,
        ],
    };
