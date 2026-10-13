use margaret_attributes::framework_vocabulary::FrameworkVocabulary;
use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;

use crate::declared_client_key_source::DeclaredClientKeySource;
use crate::declared_consent::DeclaredConsent;
use crate::declared_id_token_signing::DeclaredIdTokenSigning;

fn client_keys_name(keys: DeclaredClientKeySource) -> &'static str {
    match keys {
        DeclaredClientKeySource::Own => "Own",
        DeclaredClientKeySource::Published => "Published",
    }
}

fn consent_policy_name(consent: DeclaredConsent) -> &'static str {
    match consent {
        DeclaredConsent::Implicit => "Implicit",
        DeclaredConsent::Prompted => "Prompted",
    }
}

fn id_token_signing_name(signing: DeclaredIdTokenSigning) -> &'static str {
    match signing {
        DeclaredIdTokenSigning::EllipticCurve => "EllipticCurve",
        DeclaredIdTokenSigning::Rsa => "Rsa",
    }
}

fn jws_algorithm_name(algorithm: JwsAlgorithm) -> &'static str {
    match algorithm {
        JwsAlgorithm::Es256 => "Es256",
        JwsAlgorithm::Es384 => "Es384",
        JwsAlgorithm::Es512 => "Es512",
        JwsAlgorithm::EdDsa => "EdDsa",
        JwsAlgorithm::Ed25519 => "Ed25519",
        JwsAlgorithm::Ps256 => "Ps256",
        JwsAlgorithm::Ps384 => "Ps384",
        JwsAlgorithm::Ps512 => "Ps512",
        JwsAlgorithm::Rs256 => "Rs256",
        JwsAlgorithm::Rs384 => "Rs384",
        JwsAlgorithm::Rs512 => "Rs512",
    }
}

pub const ACCEPTED_CLIENTS_MODULE_NAME: &str = "accepted_clients";

pub(crate) const CLIENTS_MODULE_NAME: &str = "clients";

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

pub(crate) const CLIENT_SIGNING_ALGORITHMS: FrameworkVocabulary<JwsAlgorithm> =
    FrameworkVocabulary {
        enum_path: &[
            "margaret",
            "framework",
            "jose_parameters",
            "jws_algorithm",
            "JwsAlgorithm",
        ],
        name: jws_algorithm_name,
        variants: &JwsAlgorithm::ALL,
    };

pub(crate) const CONSENT_POLICIES: FrameworkVocabulary<DeclaredConsent> = FrameworkVocabulary {
    enum_path: &[
        "margaret",
        "framework",
        "accepted_clients",
        "consent_policy",
        "ConsentPolicy",
    ],
    name: consent_policy_name,
    variants: &[DeclaredConsent::Implicit, DeclaredConsent::Prompted],
};

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
