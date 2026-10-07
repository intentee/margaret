use std::collections::BTreeSet;

use proc_macro2::Ident;
use uuid::Uuid;

use margaret_attribute_arguments::attribute_arguments_reader::AttributeArgumentsReader;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::matched_attribute::MatchedAttribute;
use margaret_declaration_anchor::declaration_anchor::declaration_anchor;
use margaret_declaration_anchor::declared_anchor::DeclaredAnchor;
use margaret_https_url::https_url::HttpsUrl;
use margaret_oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod;
use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_oauth_vocabulary::grant_type::GrantType;
use margaret_oauth_vocabulary::resource_scope::ResourceScope;
use margaret_oauth_vocabulary::scope::Scope;
use margaret_registered_claims::audience::Audience;
use margaret_token_issuance_codegen::token_issuance_declaration::TokenIssuanceDeclaration;

use crate::accepted_clients_codegen_error::AcceptedClientsCodegenError;
use crate::declared_accepted_authentication::DeclaredAcceptedAuthentication;
use crate::declared_client_credentials::DeclaredClientCredentials;
use crate::declared_code_grant::DeclaredCodeGrant;
use crate::declared_code_policy::DeclaredCodePolicy;
use crate::declared_confidential_client::DeclaredConfidentialClient;
use crate::declared_consent::DeclaredConsent;
use crate::declared_id_token_signing::DeclaredIdTokenSigning;
use crate::declared_redirect_uri::declared_redirect_uri;

fn client_credentials(
    reader: &mut AttributeArgumentsReader,
    anchor: &str,
) -> Result<DeclaredClientCredentials, AcceptedClientsCodegenError> {
    Ok(reader
        .take_group("client_credentials", |group| {
            group
                .take_string_array("scopes")?
                .ok_or_else(
                    || AcceptedClientsCodegenError::MissingClientCredentialsScopes {
                        anchor: anchor.to_string(),
                    },
                )?
                .iter()
                .map(|scope| {
                    scope.parse::<ResourceScope>().map_err(|source| {
                        AcceptedClientsCodegenError::MalformedClientCredentialsScope {
                            anchor: anchor.to_string(),
                            source,
                        }
                    })
                })
                .collect::<Result<Vec<ResourceScope>, AcceptedClientsCodegenError>>()
        })?
        .map_or(DeclaredClientCredentials::Withheld, |scopes| {
            DeclaredClientCredentials::Granted { scopes }
        }))
}

fn declared_authentication(
    keyword: &Ident,
    reader: &mut AttributeArgumentsReader,
    anchor: &str,
) -> Result<DeclaredAcceptedAuthentication, AcceptedClientsCodegenError> {
    let method = keyword
        .to_string()
        .parse::<ClientAuthenticationMethod>()
        .map_err(
            |source| AcceptedClientsCodegenError::MalformedAuthentication {
                anchor: anchor.to_string(),
                source,
            },
        )?;

    match method {
        ClientAuthenticationMethod::ClientSecretBasic => {
            Err(AcceptedClientsCodegenError::UnsupportedAuthentication {
                anchor: anchor.to_string(),
                method: method.wire_name(),
            })
        }
        ClientAuthenticationMethod::None => Ok(DeclaredAcceptedAuthentication::Public),
        ClientAuthenticationMethod::PrivateKeyJwt => Ok(
            DeclaredAcceptedAuthentication::PrivateKeyJwt(DeclaredConfidentialClient {
                client_credentials: client_credentials(reader, anchor)?,
                introspection: reader.take_flag("introspection"),
                jwks_uri: reader
                    .take_string("jwks_uri")?
                    .ok_or_else(|| AcceptedClientsCodegenError::MissingJwksUri {
                        anchor: anchor.to_string(),
                    })?
                    .parse::<HttpsUrl>()
                    .map_err(|source| AcceptedClientsCodegenError::MalformedJwksUri {
                        anchor: anchor.to_string(),
                        source,
                    })?,
            }),
        ),
    }
}

fn code_policy(
    reader: &mut AttributeArgumentsReader,
    anchor: &str,
) -> Result<DeclaredCodePolicy, AcceptedClientsCodegenError> {
    let consent = reader
        .take_keyword("consent", |keyword, _| {
            DeclaredConsent::of(keyword).ok_or_else(|| {
                AcceptedClientsCodegenError::MalformedConsent {
                    anchor: anchor.to_string(),
                    keyword: keyword.to_string(),
                }
            })
        })?
        .ok_or_else(|| AcceptedClientsCodegenError::MissingConsent {
            anchor: anchor.to_string(),
        })?;
    let id_token_signing = reader
        .take_keyword("id_token_signing", |keyword, _| {
            DeclaredIdTokenSigning::of(keyword).ok_or_else(|| {
                AcceptedClientsCodegenError::MalformedIdTokenSigning {
                    anchor: anchor.to_string(),
                    keyword: keyword.to_string(),
                }
            })
        })?
        .ok_or_else(|| AcceptedClientsCodegenError::MissingIdTokenSigning {
            anchor: anchor.to_string(),
        })?;
    let redirect_uris = reader.take_string_array("redirect_uris")?.ok_or_else(|| {
        AcceptedClientsCodegenError::MissingRedirectUris {
            anchor: anchor.to_string(),
        }
    })?;

    if redirect_uris.is_empty() {
        return Err(AcceptedClientsCodegenError::EmptyRedirectUris {
            anchor: anchor.to_string(),
        });
    }

    Ok(DeclaredCodePolicy {
        consent,
        id_token_signing,
        redirect_uris: redirect_uris
            .into_iter()
            .map(|redirect_uri| declared_redirect_uri(redirect_uri, anchor))
            .collect::<Result<_, _>>()?,
        refresh_token: reader.take_flag("refresh_token"),
        scopes: reader
            .take_string_array("scopes")?
            .ok_or_else(|| AcceptedClientsCodegenError::MissingCodeScopes {
                anchor: anchor.to_string(),
            })?
            .iter()
            .map(|scope| {
                scope.parse::<Scope>().map_err(|source| {
                    AcceptedClientsCodegenError::MalformedCodeScope {
                        anchor: anchor.to_string(),
                        source,
                    }
                })
            })
            .collect::<Result<_, _>>()?,
    })
}

fn client_id(
    reader: &mut AttributeArgumentsReader,
    anchor: &str,
) -> Result<ClientId, AcceptedClientsCodegenError> {
    let client_id = reader
        .take_string("client_id")?
        .ok_or_else(|| AcceptedClientsCodegenError::MissingClientId {
            anchor: anchor.to_string(),
        })?
        .parse::<ClientId>()
        .map_err(|source| AcceptedClientsCodegenError::MalformedClientId {
            anchor: anchor.to_string(),
            source,
        })?;

    if Uuid::try_parse(client_id.as_str()).is_ok() {
        return Err(AcceptedClientsCodegenError::UuidClientId {
            anchor: anchor.to_string(),
            client_id: client_id.to_string(),
        });
    }

    Ok(client_id)
}

fn resources(
    reader: &mut AttributeArgumentsReader,
    anchor: &str,
) -> Result<Vec<Audience>, AcceptedClientsCodegenError> {
    let resources = reader.take_string_array("resources")?.ok_or_else(|| {
        AcceptedClientsCodegenError::MissingResources {
            anchor: anchor.to_string(),
        }
    })?;

    if resources.is_empty() {
        return Err(AcceptedClientsCodegenError::EmptyResources {
            anchor: anchor.to_string(),
        });
    }

    resources
        .iter()
        .map(|resource| {
            resource.parse::<Audience>().map_err(|source| {
                AcceptedClientsCodegenError::MalformedResource {
                    anchor: anchor.to_string(),
                    source,
                }
            })
        })
        .collect()
}

pub struct AcceptedClientDeclaration<'index> {
    pub anchor: DeclaredAnchor<'index>,
    pub authentication: DeclaredAcceptedAuthentication,
    pub authorization_code: DeclaredCodeGrant,
    pub client_id: ClientId,
    pub resources: Vec<Audience>,
    pub token_exchange: bool,
}

impl<'index> AcceptedClientDeclaration<'index> {
    #[must_use]
    pub fn grant_types(&self) -> BTreeSet<GrantType> {
        let mut grant_types = BTreeSet::new();

        if let DeclaredCodeGrant::Granted(policy) = &self.authorization_code {
            grant_types.insert(GrantType::AuthorizationCode);

            if policy.refresh_token {
                grant_types.insert(GrantType::RefreshToken);
            }
        }

        if let DeclaredAcceptedAuthentication::PrivateKeyJwt(DeclaredConfidentialClient {
            client_credentials: DeclaredClientCredentials::Granted { .. },
            ..
        }) = &self.authentication
        {
            grant_types.insert(GrantType::ClientCredentials);
        }

        if self.token_exchange {
            grant_types.insert(GrantType::TokenExchange);
        }

        grant_types
    }

    #[must_use]
    pub fn path(&self) -> String {
        self.anchor.item.canonical_path().to_string()
    }

    #[must_use]
    pub fn scopes(&self) -> BTreeSet<&str> {
        let mut scopes = BTreeSet::new();

        if let DeclaredCodeGrant::Granted(policy) = &self.authorization_code {
            scopes.extend(policy.scopes.iter().map(Scope::as_str));
        }

        if let DeclaredAcceptedAuthentication::PrivateKeyJwt(DeclaredConfidentialClient {
            client_credentials: DeclaredClientCredentials::Granted { scopes: granted },
            ..
        }) = &self.authentication
        {
            scopes.extend(granted.iter().map(ResourceScope::as_str));
        }

        scopes
    }

    pub(crate) fn admitted_by(
        &self,
        issuance: &TokenIssuanceDeclaration,
    ) -> Result<(), AcceptedClientsCodegenError> {
        if self.client_id.as_str() == issuance.issuer.as_str() {
            return Err(AcceptedClientsCodegenError::IssuerClientId {
                anchor: self.path(),
                client_id: self.client_id.to_string(),
            });
        }

        if self.resources.contains(&issuance.audience) {
            return Err(AcceptedClientsCodegenError::SessionAudienceResource {
                anchor: self.path(),
                audience: issuance.audience.to_string(),
            });
        }

        Ok(())
    }

    pub(crate) fn read(
        index: &'index AttributeIndex,
        matched: &MatchedAttribute<'index>,
    ) -> Result<Self, AcceptedClientsCodegenError> {
        let anchor = declaration_anchor(index, matched, FrameworkAttribute::AcceptsOAuthClient)?;
        let path = anchor.item.canonical_path().to_string();

        matched.args()?.interpret(|reader| {
            Ok(Self {
                authentication: reader
                    .take_keyword("authentication", |keyword, arguments| {
                        declared_authentication(keyword, arguments, &path)
                    })?
                    .ok_or_else(|| AcceptedClientsCodegenError::MissingAuthentication {
                        anchor: path.clone(),
                    })?,
                authorization_code: reader
                    .take_group("authorization_code", |group| code_policy(group, &path))?
                    .map_or(DeclaredCodeGrant::Withheld, DeclaredCodeGrant::Granted),
                client_id: client_id(reader, &path)?,
                resources: resources(reader, &path)?,
                token_exchange: reader.take_flag("token_exchange"),
                anchor,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use std::mem::discriminant;

    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_attributes::attribute_error::AttributeError;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_oauth_vocabulary::grant_type::GrantType;
    use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

    use crate::accepted_clients_codegen_error::AcceptedClientsCodegenError;
    use crate::declared_accepted_authentication::DeclaredAcceptedAuthentication;
    use crate::declared_accepted_clients::DeclaredAcceptedClients;
    use crate::declared_client_credentials::DeclaredClientCredentials;
    use crate::declared_code_grant::DeclaredCodeGrant;
    use crate::declared_consent::DeclaredConsent;
    use crate::declared_id_token_signing::DeclaredIdTokenSigning;

    const ANCHOR: &str = "crate::Client";
    const ISSUANCE: &str = "#[issues_tokens(audience = \"session\", issuer = \"https://issuer.example\")]\npub struct Issuer;\n";
    const PRIVATE_KEY_JWT: &str = "authentication = private_key_jwt(client_credentials(scopes = [\"artifacts:read\"]), introspection, jwks_uri = \"https://portal.example/jwks.json\")";
    const AUTHORIZATION_CODE: &str = "authorization_code(consent = prompted, id_token_signing = rsa, redirect_uris = [\"https://portal.example/callback\"], scopes = [\"openid\", \"profile\"], refresh_token)";

    fn read<TOutcome>(
        arguments: &str,
        outcome: impl FnOnce(Result<DeclaredAcceptedClients, AcceptedClientsCodegenError>) -> TOutcome,
    ) -> TOutcome {
        let indexed = IndexedSource::new(&format!(
            "{ISSUANCE}#[accepts_oauth_client({arguments})]\npub struct Client;\n"
        ));
        let issuance = DeclaredTokenIssuance::read(&indexed.index).expect("the issuance is read");

        outcome(DeclaredAcceptedClients::read(&indexed.index, &issuance))
    }

    fn rejection(arguments: &str) -> AcceptedClientsCodegenError {
        read(arguments, |read| {
            read.err().expect("the declaration is rejected")
        })
    }

    #[test]
    fn reads_a_confidential_client() {
        read(
            &format!(
                "{PRIVATE_KEY_JWT}, {AUTHORIZATION_CODE}, client_id = \"portal\", resources = [\"artifacts\", \"reports\"], token_exchange"
            ),
            |read| {
                let accepted = read.expect("the client is accepted");
                let client = accepted.clients.first().expect("the client is accepted");

                assert_eq!(accepted.clients.len(), 1);
                assert!(matches!(
                    &client.authentication,
                    DeclaredAcceptedAuthentication::PrivateKeyJwt(confidential)
                        if confidential.introspection
                            && confidential.jwks_uri.as_str() == "https://portal.example/jwks.json"
                            && matches!(
                                &confidential.client_credentials,
                                DeclaredClientCredentials::Granted { scopes } if scopes.len() == 1
                            )
                ));
                assert!(matches!(
                    &client.authorization_code,
                    DeclaredCodeGrant::Granted(policy)
                        if policy.consent == DeclaredConsent::Prompted
                            && policy.id_token_signing == DeclaredIdTokenSigning::Rsa
                            && policy.refresh_token
                            && policy.redirect_uris.len() == 1
                            && policy.scopes.len() == 2
                ));
                assert_eq!(client.client_id.as_str(), "portal");
                assert_eq!(client.resources.len(), 2);
                assert!(client.token_exchange);
                assert_eq!(client.anchor.identifier.field(), "client");
            },
        );
    }

    #[test]
    fn reads_a_public_client_without_grants() {
        read(
            "authentication = none, client_id = \"spa\", resources = [\"artifacts\"]",
            |read| {
                let accepted = read.expect("the client is accepted");
                let client = accepted.clients.first().expect("the client is accepted");

                assert_eq!(
                    discriminant(&client.authentication),
                    discriminant(&DeclaredAcceptedAuthentication::Public)
                );
                assert_eq!(
                    discriminant(&client.authorization_code),
                    discriminant(&DeclaredCodeGrant::Withheld)
                );
                assert!(!client.token_exchange);
                assert!(client.grant_types().is_empty());
                assert!(client.scopes().is_empty());
            },
        );
    }

    #[test]
    fn grants_the_types_and_scopes_it_declares() {
        read(
            &format!(
                "{PRIVATE_KEY_JWT}, {AUTHORIZATION_CODE}, client_id = \"portal\", resources = [\"artifacts\"], token_exchange"
            ),
            |read| {
                let accepted = read.expect("the client is accepted");
                let client = accepted.clients.first().expect("the client is accepted");

                assert_eq!(
                    client.grant_types().into_iter().collect::<Vec<GrantType>>(),
                    GrantType::ALL.to_vec()
                );
                assert_eq!(
                    client.scopes().into_iter().collect::<Vec<&str>>(),
                    vec!["artifacts:read", "openid", "profile"]
                );
            },
        );
    }

    #[test]
    fn reads_a_confidential_client_without_its_optional_privileges() {
        read(
            "authentication = private_key_jwt(jwks_uri = \"https://portal.example/jwks.json\"), client_id = \"portal\", resources = [\"artifacts\"]",
            |read| {
                let accepted = read.expect("the client is accepted");
                let client = accepted.clients.first().expect("the client is accepted");

                assert!(matches!(
                    &client.authentication,
                    DeclaredAcceptedAuthentication::PrivateKeyJwt(confidential)
                        if !confidential.introspection
                            && discriminant(&confidential.client_credentials)
                                == discriminant(&DeclaredClientCredentials::Withheld)
                ));
            },
        );
    }

    #[test]
    fn rejects_a_client_without_authentication() {
        assert!(matches!(
            rejection(
                "client_id = \"spa\", resources = [\"artifacts\"]"
            ),
            AcceptedClientsCodegenError::MissingAuthentication { anchor, .. }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_an_authentication_method_outside_the_vocabulary() {
        assert!(matches!(
            rejection(
                "authentication = password, client_id = \"spa\", resources = [\"artifacts\"]"
            ),
            AcceptedClientsCodegenError::MalformedAuthentication { anchor, .. }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_a_client_authenticating_with_a_shared_secret() {
        assert!(matches!(
            rejection(
                "authentication = client_secret_basic, client_id = \"spa\", resources = [\"artifacts\"]"
            ),
            AcceptedClientsCodegenError::UnsupportedAuthentication { anchor, method }
                if anchor == ANCHOR && method == "client_secret_basic"
        ));
    }

    #[test]
    fn rejects_privileges_of_a_public_client() {
        assert!(matches!(
            rejection(
                "authentication = none(introspection), client_id = \"spa\", resources = [\"artifacts\"]"
            ),
            AcceptedClientsCodegenError::AttributeArguments(AttributeArgumentsError::UnrecognizedArgument { argument, .. })
                if argument == "introspection"
        ));
    }

    #[test]
    fn rejects_a_confidential_client_without_its_jwks_uri() {
        assert!(matches!(
            rejection(
                "authentication = private_key_jwt(introspection), client_id = \"portal\", resources = [\"artifacts\"]"
            ),
            AcceptedClientsCodegenError::MissingJwksUri { anchor, .. }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_a_jwks_uri_that_is_not_a_string() {
        assert!(matches!(
            rejection("authentication = private_key_jwt(jwks_uri = jwks), client_id = \"portal\", resources = [\"artifacts\"]"),
            AcceptedClientsCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "jwks_uri"
        ));
    }

    #[test]
    fn rejects_a_jwks_uri_that_is_not_https() {
        assert!(matches!(
            rejection(
                "authentication = private_key_jwt(jwks_uri = \"http://portal.example/jwks.json\"), client_id = \"portal\", resources = [\"artifacts\"]"
            ),
            AcceptedClientsCodegenError::MalformedJwksUri { anchor, .. }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_client_credentials_without_their_scopes() {
        assert!(matches!(
            rejection(
                "authentication = private_key_jwt(client_credentials(), jwks_uri = \"https://portal.example/jwks.json\"), client_id = \"portal\", resources = [\"artifacts\"]"
            ),
            AcceptedClientsCodegenError::MissingClientCredentialsScopes { anchor, .. }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_client_credentials_scopes_that_are_not_an_array() {
        assert!(matches!(
            rejection("authentication = private_key_jwt(client_credentials(scopes = \"profile\"), jwks_uri = \"https://portal.example/jwks.json\"), client_id = \"portal\", resources = [\"artifacts\"]"),
            AcceptedClientsCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "scopes"
        ));
    }

    #[test]
    fn rejects_openid_as_a_client_credentials_scope() {
        assert!(matches!(
            rejection(
                "authentication = private_key_jwt(client_credentials(scopes = [\"openid\"]), jwks_uri = \"https://portal.example/jwks.json\"), client_id = \"portal\", resources = [\"artifacts\"]"
            ),
            AcceptedClientsCodegenError::MalformedClientCredentialsScope { anchor, .. }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_an_authorization_code_grant_without_its_consent() {
        assert!(matches!(
            rejection(
                "authentication = none, authorization_code(id_token_signing = rsa, redirect_uris = [\"https://spa.example/callback\"], scopes = []), client_id = \"spa\", resources = [\"artifacts\"]"
            ),
            AcceptedClientsCodegenError::MissingConsent { anchor, .. }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_a_consent_outside_the_grammar() {
        assert!(matches!(
            rejection(
                "authentication = none, authorization_code(consent = never, id_token_signing = rsa, redirect_uris = [\"https://spa.example/callback\"], scopes = []), client_id = \"spa\", resources = [\"artifacts\"]"
            ),
            AcceptedClientsCodegenError::MalformedConsent { anchor, keyword }
                if anchor == ANCHOR && keyword == "never"
        ));
    }

    #[test]
    fn rejects_an_authorization_code_grant_without_its_id_token_signing() {
        assert!(matches!(
            rejection(
                "authentication = none, authorization_code(consent = implicit, redirect_uris = [\"https://spa.example/callback\"], scopes = []), client_id = \"spa\", resources = [\"artifacts\"]"
            ),
            AcceptedClientsCodegenError::MissingIdTokenSigning { anchor, .. }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_an_id_token_signing_outside_the_grammar() {
        assert!(matches!(
            rejection(
                "authentication = none, authorization_code(consent = implicit, id_token_signing = hmac, redirect_uris = [\"https://spa.example/callback\"], scopes = []), client_id = \"spa\", resources = [\"artifacts\"]"
            ),
            AcceptedClientsCodegenError::MalformedIdTokenSigning { anchor, keyword }
                if anchor == ANCHOR && keyword == "hmac"
        ));
    }

    #[test]
    fn rejects_an_authorization_code_grant_without_its_redirect_uris() {
        assert!(matches!(
            rejection(
                "authentication = none, authorization_code(consent = implicit, id_token_signing = elliptic_curve, scopes = []), client_id = \"spa\", resources = [\"artifacts\"]"
            ),
            AcceptedClientsCodegenError::MissingRedirectUris { anchor, .. }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_redirect_uris_that_are_not_an_array() {
        assert!(matches!(
            rejection("authentication = none, authorization_code(consent = implicit, id_token_signing = elliptic_curve, redirect_uris = \"https://spa.example/callback\", scopes = []), client_id = \"spa\", resources = [\"artifacts\"]"),
            AcceptedClientsCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "redirect_uris"
        ));
    }

    #[test]
    fn rejects_an_empty_list_of_redirect_uris() {
        assert!(matches!(
            rejection(
                "authentication = none, authorization_code(consent = implicit, id_token_signing = elliptic_curve, redirect_uris = [], scopes = []), client_id = \"spa\", resources = [\"artifacts\"]"
            ),
            AcceptedClientsCodegenError::EmptyRedirectUris { anchor, .. }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_an_insecure_redirect_uri() {
        assert!(matches!(
            rejection(
                "authentication = none, authorization_code(consent = implicit, id_token_signing = elliptic_curve, redirect_uris = [\"http://spa.example/callback\"], scopes = []), client_id = \"spa\", resources = [\"artifacts\"]"
            ),
            AcceptedClientsCodegenError::InsecureRedirectUri { anchor, redirect_uri }
                if anchor == ANCHOR && redirect_uri == "http://spa.example/callback"
        ));
    }

    #[test]
    fn rejects_an_authorization_code_grant_without_its_scopes() {
        assert!(matches!(
            rejection(
                "authentication = none, authorization_code(consent = implicit, id_token_signing = elliptic_curve, redirect_uris = [\"https://spa.example/callback\"]), client_id = \"spa\", resources = [\"artifacts\"]"
            ),
            AcceptedClientsCodegenError::MissingCodeScopes { anchor, .. }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_authorization_code_scopes_that_are_not_an_array() {
        assert!(matches!(
            rejection("authentication = none, authorization_code(consent = implicit, id_token_signing = elliptic_curve, redirect_uris = [\"https://spa.example/callback\"], scopes = \"openid\"), client_id = \"spa\", resources = [\"artifacts\"]"),
            AcceptedClientsCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "scopes"
        ));
    }

    #[test]
    fn rejects_a_malformed_authorization_code_scope() {
        assert!(matches!(
            rejection(
                "authentication = none, authorization_code(consent = implicit, id_token_signing = elliptic_curve, redirect_uris = [\"https://spa.example/callback\"], scopes = [\"a\\\"b\"]), client_id = \"spa\", resources = [\"artifacts\"]"
            ),
            AcceptedClientsCodegenError::MalformedCodeScope { anchor, .. }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_a_client_without_a_client_id() {
        assert!(matches!(
            rejection(
                "authentication = none, resources = [\"artifacts\"]"
            ),
            AcceptedClientsCodegenError::MissingClientId { anchor, .. }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_a_client_id_that_is_not_a_string() {
        assert!(matches!(
            rejection("authentication = none, client_id = spa, resources = [\"artifacts\"]"),
            AcceptedClientsCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "client_id"
        ));
    }

    #[test]
    fn rejects_a_client_id_with_invisible_characters() {
        assert!(matches!(
            rejection(
                "authentication = none, client_id = \"s\\tpa\", resources = [\"artifacts\"]"
            ),
            AcceptedClientsCodegenError::MalformedClientId { anchor, .. }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_a_client_identified_by_a_uuid() {
        assert!(matches!(
            rejection(
                "authentication = none, client_id = \"67e55044-10b1-426f-9247-bb680e5fe0c8\", resources = [\"artifacts\"]"
            ),
            AcceptedClientsCodegenError::UuidClientId { anchor, client_id }
                if anchor == ANCHOR && client_id == "67e55044-10b1-426f-9247-bb680e5fe0c8"
        ));
    }

    #[test]
    fn rejects_a_client_identified_by_the_issuer() {
        assert!(matches!(
            rejection(
                "authentication = none, client_id = \"https://issuer.example\", resources = [\"artifacts\"]"
            ),
            AcceptedClientsCodegenError::IssuerClientId { anchor, client_id }
                if anchor == ANCHOR && client_id == "https://issuer.example"
        ));
    }

    #[test]
    fn rejects_a_client_without_resources() {
        assert!(matches!(
            rejection(
                "authentication = none, client_id = \"spa\""
            ),
            AcceptedClientsCodegenError::MissingResources { anchor, .. }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_resources_that_are_not_an_array() {
        assert!(matches!(
            rejection("authentication = none, client_id = \"spa\", resources = \"artifacts\""),
            AcceptedClientsCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "resources"
        ));
    }

    #[test]
    fn rejects_an_empty_list_of_resources() {
        assert!(matches!(
            rejection(
                "authentication = none, client_id = \"spa\", resources = []"
            ),
            AcceptedClientsCodegenError::EmptyResources { anchor, .. }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_a_malformed_resource() {
        assert!(matches!(
            rejection(
                "authentication = none, client_id = \"spa\", resources = [\"\"]"
            ),
            AcceptedClientsCodegenError::MalformedResource { anchor, .. }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_a_client_naming_the_session_audience_as_a_resource() {
        assert!(matches!(
            rejection(
                "authentication = none, client_id = \"spa\", resources = [\"session\"]"
            ),
            AcceptedClientsCodegenError::SessionAudienceResource { anchor, audience }
                if anchor == ANCHOR && audience == "session"
        ));
    }

    #[test]
    fn reports_unparseable_client_arguments() {
        assert!(matches!(
            rejection(
                "= 5"
            ),
            AcceptedClientsCodegenError::Index(AttributeError::Arguments(AttributeArgumentsError::Malformed { attribute_path, .. }))
                if attribute_path == "accepts_oauth_client"
        ));
    }
}
