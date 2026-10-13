use margaret_attributes::framework_vocabulary::FrameworkVocabulary;
use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;

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
