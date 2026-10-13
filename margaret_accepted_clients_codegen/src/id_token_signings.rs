use margaret_attributes::framework_vocabulary::FrameworkVocabulary;

use crate::declared_id_token_signing::DeclaredIdTokenSigning;

fn id_token_signing_name(signing: DeclaredIdTokenSigning) -> &'static str {
    match signing {
        DeclaredIdTokenSigning::EllipticCurve => "EllipticCurve",
        DeclaredIdTokenSigning::Rsa => "Rsa",
    }
}

pub(crate) const ID_TOKEN_SIGNINGS: FrameworkVocabulary<DeclaredIdTokenSigning> =
    FrameworkVocabulary {
        enum_path: &[
            "margaret",
            "framework",
            "jwks_secret_store",
            "id_token_signing",
            "IdTokenSigning",
        ],
        name: id_token_signing_name,
        variants: &[
            DeclaredIdTokenSigning::EllipticCurve,
            DeclaredIdTokenSigning::Rsa,
        ],
    };
