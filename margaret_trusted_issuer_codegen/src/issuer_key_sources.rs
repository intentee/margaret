use margaret_attributes::framework_vocabulary::FrameworkVocabulary;

use crate::declared_issuer_keys::DeclaredIssuerKeys;

fn issuer_keys_name(keys: DeclaredIssuerKeys) -> &'static str {
    match keys {
        DeclaredIssuerKeys::Discovered => "Discovered",
        DeclaredIssuerKeys::Published => "Published",
    }
}

pub(crate) const ISSUER_KEY_SOURCES: FrameworkVocabulary<DeclaredIssuerKeys> =
    FrameworkVocabulary {
        enum_path: &[
            "margaret",
            "framework",
            "trusted_issuer",
            "issuer_keys",
            "IssuerKeys",
        ],
        name: issuer_keys_name,
        variants: &[
            DeclaredIssuerKeys::Discovered,
            DeclaredIssuerKeys::Published,
        ],
    };
