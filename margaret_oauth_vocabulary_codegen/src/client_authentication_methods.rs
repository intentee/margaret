use margaret_attributes::framework_vocabulary::FrameworkVocabulary;
use margaret_oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod;

fn client_authentication_method_name(method: ClientAuthenticationMethod) -> &'static str {
    match method {
        ClientAuthenticationMethod::ClientSecretBasic => "ClientSecretBasic",
        ClientAuthenticationMethod::None => "None",
        ClientAuthenticationMethod::PrivateKeyJwt => "PrivateKeyJwt",
    }
}

pub const CLIENT_AUTHENTICATION_METHODS: FrameworkVocabulary<ClientAuthenticationMethod> =
    FrameworkVocabulary {
        enum_path: &[
            "margaret",
            "framework",
            "oauth_vocabulary",
            "client_authentication_method",
            "ClientAuthenticationMethod",
        ],
        name: client_authentication_method_name,
        variants: &ClientAuthenticationMethod::ALL,
    };

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod;

    use super::CLIENT_AUTHENTICATION_METHODS;

    #[test]
    fn reads_every_method_by_its_framework_variant() {
        let read: Vec<Option<ClientAuthenticationMethod>> =
            ["ClientSecretBasic", "None", "PrivateKeyJwt"]
                .into_iter()
                .map(|variant| {
                    CLIENT_AUTHENTICATION_METHODS.variant(&CanonicalPath::new(
                        [
                            "margaret",
                            "framework",
                            "oauth_vocabulary",
                            "client_authentication_method",
                            "ClientAuthenticationMethod",
                            variant,
                        ]
                        .map(ToString::to_string)
                        .to_vec(),
                    ))
                })
                .collect();

        assert_eq!(
            read,
            vec![
                Some(ClientAuthenticationMethod::ClientSecretBasic),
                Some(ClientAuthenticationMethod::None),
                Some(ClientAuthenticationMethod::PrivateKeyJwt),
            ]
        );
    }
}
