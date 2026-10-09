use margaret_attributes::framework_vocabulary::FrameworkVocabulary;

use crate::declared_client_key_source::DeclaredClientKeySource;

fn client_keys_name(keys: DeclaredClientKeySource) -> &'static str {
    match keys {
        DeclaredClientKeySource::Own => "Own",
        DeclaredClientKeySource::Published => "Published",
    }
}

pub(crate) const CLIENT_KEY_SOURCES: FrameworkVocabulary<DeclaredClientKeySource> =
    FrameworkVocabulary {
        enum_path: &[
            "margaret",
            "framework",
            "accepted_clients",
            "client_keys",
            "ClientKeys",
        ],
        name: client_keys_name,
        variants: &[
            DeclaredClientKeySource::Own,
            DeclaredClientKeySource::Published,
        ],
    };
