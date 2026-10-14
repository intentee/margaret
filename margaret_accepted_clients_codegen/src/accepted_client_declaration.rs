use std::collections::BTreeSet;

use quote::ToTokens;
use syn::Path;
use url::Url;
use uuid::Uuid;

use margaret_attribute_arguments::attribute_arguments_reader::AttributeArgumentsReader;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::matched_attribute::MatchedAttribute;
use margaret_attributes::tag::Tag;
use margaret_declaration_anchor::declaration_anchor::declaration_anchor;
use margaret_declaration_anchor::declared_anchor::DeclaredAnchor;
use margaret_https_url::https_url::HttpsUrl;
use margaret_https_url::https_url_parsing::HttpsUrlParsing;
use margaret_item_naming_argument::item_naming_argument::ItemNamingArgument;
use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod;
use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_oauth_vocabulary::client_id_parsing::ClientIdParsing;
use margaret_oauth_vocabulary::grant_type::GrantType;
use margaret_oauth_vocabulary::resource_scope::ResourceScope;
use margaret_oauth_vocabulary::resource_scope_parsing::ResourceScopeParsing;
use margaret_oauth_vocabulary::scope::Scope;
use margaret_oauth_vocabulary_codegen::constants::CLIENT_AUTHENTICATION_METHODS;
use margaret_oauth_vocabulary_codegen::declared_scopes::DeclaredScopes;
use margaret_oauth_vocabulary_codegen::scope_list_resolution::ScopeListResolution;
use margaret_token_issuance_codegen::declared_resource_issuances::DeclaredResourceIssuances;
use margaret_token_issuance_codegen::token_issuance_declaration::TokenIssuanceDeclaration;

use crate::accepted_clients_codegen_error::AcceptedClientsCodegenError;
use crate::admitted_resource::AdmittedResource;
use crate::client_resources::ClientResources;
use crate::constants::CLIENT_KEY_SOURCES;
use crate::constants::CLIENT_SIGNING_ALGORITHMS;
use crate::constants::CONSENT_POLICIES;
use crate::constants::ID_TOKEN_SIGNINGS;
use crate::declared_accepted_authentication::DeclaredAcceptedAuthentication;
use crate::declared_client_credentials::DeclaredClientCredentials;
use crate::declared_client_key_source::DeclaredClientKeySource;
use crate::declared_client_keys::DeclaredClientKeys;
use crate::declared_code_grant::DeclaredCodeGrant;
use crate::declared_code_policy::DeclaredCodePolicy;
use crate::declared_confidential_client::DeclaredConfidentialClient;
use crate::declared_redirect_uri::declared_redirect_uri;
use crate::declared_token_exchange::DeclaredTokenExchange;

fn written(path: &Path) -> String {
    path.to_token_stream().to_string()
}

fn scope_list(
    context: &DeclarationContext,
    reader: &mut AttributeArgumentsReader,
) -> Result<Option<Vec<Scope>>, AcceptedClientsCodegenError> {
    reader
        .take_path_array("scopes")?
        .map(|written_scopes| {
            match context
                .scopes
                .resolve_list(context.index, context.item, &written_scopes)
            {
                ScopeListResolution::Declared(scopes) => Ok(scopes),
                ScopeListResolution::Repeated(written_scope) => {
                    Err(AcceptedClientsCodegenError::RepeatedScope {
                        anchor: context.item.canonical_path().to_string(),
                        written: written(written_scope),
                    })
                }
                ScopeListResolution::Unknown(written_scope) => {
                    Err(AcceptedClientsCodegenError::UnknownScope {
                        anchor: context.item.canonical_path().to_string(),
                        written: written(written_scope),
                    })
                }
            }
        })
        .transpose()
}

fn client_credentials(
    context: &DeclarationContext,
    reader: &mut AttributeArgumentsReader,
    anchor: &str,
) -> Result<DeclaredClientCredentials, AcceptedClientsCodegenError> {
    Ok(reader
        .take_group("client_credentials", |group| {
            if let ClientResources::Withheld = context.resources {
                return Err(
                    AcceptedClientsCodegenError::ClientCredentialsWithoutResources {
                        anchor: anchor.to_string(),
                    },
                );
            }

            scope_list(context, group)?
                .ok_or_else(
                    || AcceptedClientsCodegenError::MissingClientCredentialsScopes {
                        anchor: anchor.to_string(),
                    },
                )?
                .iter()
                .map(|scope| match ResourceScope::parse(scope.as_str()) {
                    ResourceScopeParsing::Accepted(scope) => Ok(scope),
                    ResourceScopeParsing::Rejected(rejection) => Err(
                        AcceptedClientsCodegenError::MalformedClientCredentialsScope {
                            anchor: anchor.to_string(),
                            rejection,
                        },
                    ),
                })
                .collect::<Result<Vec<ResourceScope>, AcceptedClientsCodegenError>>()
        })?
        .map_or(DeclaredClientCredentials::Withheld, |scopes| {
            DeclaredClientCredentials::Granted { scopes }
        }))
}

fn signing_algorithm(
    index: &AttributeIndex,
    item: &IndexedItem,
    reader: &mut AttributeArgumentsReader,
) -> Result<JwsAlgorithm, AcceptedClientsCodegenError> {
    let anchor = item.canonical_path().to_string();
    let declared = reader
        .take_path(ItemNamingArgument::Signing.key())?
        .ok_or_else(|| AcceptedClientsCodegenError::MissingSigningAlgorithm {
            anchor: anchor.clone(),
        })?;

    match index
        .resolve_item_path(item, &declared)
        .as_ref()
        .and_then(|resolved| CLIENT_SIGNING_ALGORITHMS.variant(resolved))
    {
        Some(JwsAlgorithm::EdDsa) => {
            Err(AcceptedClientsCodegenError::PolymorphicSigningAlgorithm { anchor })
        }
        Some(fully_specified) => Ok(fully_specified),
        None => Err(AcceptedClientsCodegenError::UnknownSigningAlgorithm {
            anchor,
            written: written(&declared),
        }),
    }
}

fn client_keys(
    index: &AttributeIndex,
    item: &IndexedItem,
    variant: &Path,
    reader: &mut AttributeArgumentsReader,
) -> Result<DeclaredClientKeys, AcceptedClientsCodegenError> {
    let anchor = item.canonical_path().to_string();

    match index
        .resolve_item_path(item, variant)
        .as_ref()
        .and_then(|resolved| CLIENT_KEY_SOURCES.variant(resolved))
    {
        None => Err(AcceptedClientsCodegenError::UnknownClientKeys {
            anchor,
            written: written(variant),
        }),
        Some(DeclaredClientKeySource::Own) => Ok(DeclaredClientKeys::Own),
        Some(DeclaredClientKeySource::Published) => Ok(DeclaredClientKeys::Published {
            jwks_uri: match HttpsUrl::parse(&reader.take_string("jwks_uri")?.ok_or_else(|| {
                AcceptedClientsCodegenError::MissingJwksUri {
                    anchor: anchor.clone(),
                }
            })?) {
                HttpsUrlParsing::Accepted(jwks_uri) => jwks_uri,
                HttpsUrlParsing::Rejected(rejection) => {
                    return Err(AcceptedClientsCodegenError::MalformedJwksUri {
                        anchor: anchor.clone(),
                        rejection,
                    });
                }
            },
            signing: signing_algorithm(index, item, reader)?,
        }),
    }
}

fn declared_authentication(
    context: &DeclarationContext,
    variant: &Path,
    reader: &mut AttributeArgumentsReader,
) -> Result<DeclaredAcceptedAuthentication, AcceptedClientsCodegenError> {
    let DeclarationContext { index, item, .. } = *context;
    let anchor = item.canonical_path().to_string();
    let method = index
        .resolve_item_path(item, variant)
        .as_ref()
        .and_then(|resolved| CLIENT_AUTHENTICATION_METHODS.variant(resolved))
        .ok_or_else(|| AcceptedClientsCodegenError::UnknownAuthentication {
            anchor: anchor.clone(),
            written: written(variant),
        })?;

    match method {
        ClientAuthenticationMethod::ClientSecretBasic => {
            Err(AcceptedClientsCodegenError::UnsupportedAuthentication {
                anchor,
                method: method.wire_name(),
            })
        }
        ClientAuthenticationMethod::None => Ok(DeclaredAcceptedAuthentication::Public),
        ClientAuthenticationMethod::PrivateKeyJwt => Ok(
            DeclaredAcceptedAuthentication::PrivateKeyJwt(DeclaredConfidentialClient {
                client_credentials: client_credentials(context, reader, &anchor)?,
                introspection: introspection(context, reader, &anchor)?,
                keys: reader
                    .take_variant(ItemNamingArgument::Keys.key(), |keys, arguments| {
                        client_keys(index, item, keys, arguments)
                    })?
                    .ok_or_else(|| AcceptedClientsCodegenError::MissingClientKeys {
                        anchor: anchor.clone(),
                    })?,
            }),
        ),
    }
}

fn introspection(
    context: &DeclarationContext,
    reader: &mut AttributeArgumentsReader,
    anchor: &str,
) -> Result<bool, AcceptedClientsCodegenError> {
    let introspection = reader.take_flag("introspection");

    if introspection && let ClientResources::Withheld = context.resources {
        return Err(AcceptedClientsCodegenError::IntrospectionWithoutResources {
            anchor: anchor.to_string(),
        });
    }

    Ok(introspection)
}

fn redirect_uris(
    reader: &mut AttributeArgumentsReader,
    anchor: &str,
) -> Result<Vec<Url>, AcceptedClientsCodegenError> {
    let Some(declared) = reader.take_string_array("redirect_uris")? else {
        return Ok(Vec::new());
    };

    if declared.is_empty() {
        return Err(AcceptedClientsCodegenError::EmptyRedirectUris {
            anchor: anchor.to_string(),
        });
    }

    declared
        .into_iter()
        .map(|redirect_uri| declared_redirect_uri(redirect_uri, anchor))
        .collect()
}

fn redirect_routes(
    index: &AttributeIndex,
    item: &IndexedItem,
    reader: &mut AttributeArgumentsReader,
    anchor: &str,
) -> Result<Vec<CanonicalPath>, AcceptedClientsCodegenError> {
    let Some(declared) = reader.take_path_array(ItemNamingArgument::RedirectRoutes.key())? else {
        return Ok(Vec::new());
    };

    if declared.is_empty() {
        return Err(AcceptedClientsCodegenError::EmptyRedirectRoutes {
            anchor: anchor.to_string(),
        });
    }

    let mut redirect_routes: Vec<CanonicalPath> = Vec::with_capacity(declared.len());

    for route in declared {
        let resolved = index.resolve_item_path(item, &route).ok_or_else(|| {
            AcceptedClientsCodegenError::UnknownRedirectRoute {
                anchor: anchor.to_string(),
                written: written(&route),
            }
        })?;

        if redirect_routes.contains(&resolved) {
            return Err(AcceptedClientsCodegenError::DuplicateRedirectRoute {
                anchor: anchor.to_string(),
                route: resolved.to_string(),
            });
        }

        redirect_routes.push(resolved);
    }

    Ok(redirect_routes)
}

fn code_policy(
    context: &DeclarationContext,
    reader: &mut AttributeArgumentsReader,
) -> Result<DeclaredCodePolicy, AcceptedClientsCodegenError> {
    let DeclarationContext { index, item, .. } = *context;
    let anchor = item.canonical_path().to_string();
    let anchor = anchor.as_str();
    let declared_consent = reader
        .take_path(ItemNamingArgument::Consent.key())?
        .ok_or_else(|| AcceptedClientsCodegenError::MissingConsent {
            anchor: anchor.to_string(),
        })?;
    let consent = index
        .resolve_item_path(item, &declared_consent)
        .as_ref()
        .and_then(|resolved| CONSENT_POLICIES.variant(resolved))
        .ok_or_else(|| AcceptedClientsCodegenError::UnknownConsent {
            anchor: anchor.to_string(),
            written: written(&declared_consent),
        })?;
    let declared_id_token_signing = reader
        .take_path(ItemNamingArgument::IdTokenSigning.key())?
        .ok_or_else(|| AcceptedClientsCodegenError::MissingIdTokenSigning {
            anchor: anchor.to_string(),
        })?;
    let id_token_signing = index
        .resolve_item_path(item, &declared_id_token_signing)
        .as_ref()
        .and_then(|resolved| ID_TOKEN_SIGNINGS.variant(resolved))
        .ok_or_else(|| AcceptedClientsCodegenError::UnknownIdTokenSigning {
            anchor: anchor.to_string(),
            written: written(&declared_id_token_signing),
        })?;
    let redirect_routes = redirect_routes(index, item, reader, anchor)?;
    let redirect_uris = redirect_uris(reader, anchor)?;

    if redirect_routes.is_empty() && redirect_uris.is_empty() {
        return Err(AcceptedClientsCodegenError::MissingRedirectUris {
            anchor: anchor.to_string(),
        });
    }

    let refresh_token = reader.take_flag("refresh_token");
    let scopes = scope_list(context, reader)?.ok_or_else(|| {
        AcceptedClientsCodegenError::MissingCodeScopes {
            anchor: anchor.to_string(),
        }
    })?;

    if let ClientResources::Withheld = context.resources
        && !scopes.iter().any(Scope::is_openid)
    {
        return Err(
            AcceptedClientsCodegenError::ResourcelessCodeGrantWithoutOpenidScope {
                anchor: anchor.to_string(),
            },
        );
    }

    Ok(DeclaredCodePolicy {
        consent,
        id_token_signing,
        redirect_routes,
        redirect_uris,
        refresh_token,
        scopes,
    })
}

fn authorization_code(
    context: &DeclarationContext,
    reader: &mut AttributeArgumentsReader,
    anchor: &str,
) -> Result<DeclaredCodeGrant, AcceptedClientsCodegenError> {
    match (
        reader.take_group("authorization_code", |group| code_policy(context, group))?,
        context.resources,
    ) {
        (Some(policy), _) => Ok(DeclaredCodeGrant::Granted(policy)),
        (None, ClientResources::Admitted(_)) => Ok(DeclaredCodeGrant::Withheld),
        (None, ClientResources::Withheld) => Err(
            AcceptedClientsCodegenError::ResourcelessClientWithoutCodeGrant {
                anchor: anchor.to_string(),
            },
        ),
    }
}

fn client_id(
    reader: &mut AttributeArgumentsReader,
    anchor: &str,
) -> Result<ClientId, AcceptedClientsCodegenError> {
    let client_id = match ClientId::parse(&reader.take_string("client_id")?.ok_or_else(|| {
        AcceptedClientsCodegenError::MissingClientId {
            anchor: anchor.to_string(),
        }
    })?) {
        ClientIdParsing::Accepted(client_id) => client_id,
        ClientIdParsing::Rejected(rejection) => {
            return Err(AcceptedClientsCodegenError::MalformedClientId {
                anchor: anchor.to_string(),
                rejection,
            });
        }
    };

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
    resources: &DeclaredResourceIssuances,
    anchor: &str,
) -> Result<ClientResources, AcceptedClientsCodegenError> {
    let declared = reader.take_path_array("resources")?.ok_or_else(|| {
        AcceptedClientsCodegenError::MissingResources {
            anchor: anchor.to_string(),
        }
    })?;

    if declared.is_empty() {
        return Ok(ClientResources::Withheld);
    }

    let mut admitted: Vec<AdmittedResource> = Vec::with_capacity(declared.len());

    for path in &declared {
        let tag = Tag::from_path(path).ok_or_else(|| {
            AcceptedClientsCodegenError::MalformedResourceTag {
                anchor: anchor.to_string(),
            }
        })?;
        let resource =
            resources
                .find(&tag)
                .ok_or_else(|| AcceptedClientsCodegenError::UnknownResource {
                    anchor: anchor.to_string(),
                    tag: tag.to_string(),
                })?;

        if admitted.iter().any(|resource| resource.tag == tag) {
            return Err(AcceptedClientsCodegenError::DuplicateResource {
                anchor: anchor.to_string(),
                tag: tag.to_string(),
            });
        }

        admitted.push(AdmittedResource {
            audience: resource.audience.clone(),
            tag,
        });
    }

    Ok(ClientResources::Admitted(admitted))
}

fn token_exchange(
    context: &DeclarationContext,
    reader: &mut AttributeArgumentsReader,
    anchor: &str,
) -> Result<DeclaredTokenExchange, AcceptedClientsCodegenError> {
    Ok(reader
        .take_group("token_exchange", |group| {
            if let ClientResources::Withheld = context.resources {
                return Err(AcceptedClientsCodegenError::TokenExchangeWithoutResources {
                    anchor: anchor.to_string(),
                });
            }

            scope_list(context, group)?.ok_or_else(|| {
                AcceptedClientsCodegenError::MissingTokenExchangeScopes {
                    anchor: anchor.to_string(),
                }
            })
        })?
        .map_or(DeclaredTokenExchange::Withheld, |scopes| {
            DeclaredTokenExchange::Granted { scopes }
        }))
}

#[derive(Clone, Copy)]
struct DeclarationContext<'context> {
    index: &'context AttributeIndex,
    item: &'context IndexedItem,
    resources: &'context ClientResources,
    scopes: &'context DeclaredScopes<'context>,
}

pub struct AcceptedClientDeclaration<'index> {
    pub anchor: DeclaredAnchor<'index>,
    pub authentication: DeclaredAcceptedAuthentication,
    pub authorization_code: DeclaredCodeGrant,
    pub client_id: ClientId,
    pub resources: ClientResources,
    pub tag: Tag,
    pub token_exchange: DeclaredTokenExchange,
}

impl<'index> AcceptedClientDeclaration<'index> {
    pub(crate) fn read(
        index: &'index AttributeIndex,
        matched: &MatchedAttribute<'index>,
        declared_resources: &DeclaredResourceIssuances,
        scopes: &DeclaredScopes,
    ) -> Result<Self, AcceptedClientsCodegenError> {
        let anchor = declaration_anchor(index, matched, FrameworkAttribute::AdmitsOAuthClient)?;
        let item = anchor.item;
        let path = item.canonical_path().to_string();

        matched.args()?.interpret(|reader| {
            let resources = resources(reader, declared_resources, &path)?;
            let context = DeclarationContext {
                index,
                item,
                resources: &resources,
                scopes,
            };

            Ok(Self {
                tag: reader
                    .take_positional_path()
                    .ok_or_else(|| AcceptedClientsCodegenError::MissingTag {
                        anchor: path.clone(),
                    })
                    .and_then(|tag| {
                        Tag::from_path(&tag).ok_or_else(|| {
                            AcceptedClientsCodegenError::MalformedTag {
                                anchor: path.clone(),
                            }
                        })
                    })?,
                authentication: reader
                    .take_variant(
                        ItemNamingArgument::ClientAuthentication.key(),
                        |variant, arguments| declared_authentication(&context, variant, arguments),
                    )?
                    .ok_or_else(|| AcceptedClientsCodegenError::MissingAuthentication {
                        anchor: path.clone(),
                    })?,
                authorization_code: authorization_code(&context, reader, &path)?,
                client_id: client_id(reader, &path)?,
                token_exchange: token_exchange(&context, reader, &path)?,
                resources,
                anchor,
            })
        })
    }

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

        if let DeclaredTokenExchange::Granted { .. } = self.token_exchange {
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

        if let DeclaredTokenExchange::Granted { scopes: exchanged } = &self.token_exchange {
            scopes.extend(exchanged.iter().map(Scope::as_str));
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

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::mem::discriminant;

    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_attributes::attribute_error::AttributeError;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
    use margaret_oauth_vocabulary::grant_type::GrantType;
    use margaret_oauth_vocabulary::scope::Scope;
    use margaret_oauth_vocabulary_codegen::declared_scopes::DeclaredScopes;
    use margaret_token_issuance_codegen::declared_resource_issuances::DeclaredResourceIssuances;
    use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

    use crate::accepted_clients_codegen_error::AcceptedClientsCodegenError;
    use crate::client_resources::ClientResources;
    use crate::declared_accepted_authentication::DeclaredAcceptedAuthentication;
    use crate::declared_accepted_clients::DeclaredAcceptedClients;
    use crate::declared_client_credentials::DeclaredClientCredentials;
    use crate::declared_client_keys::DeclaredClientKeys;
    use crate::declared_code_grant::DeclaredCodeGrant;
    use crate::declared_consent::DeclaredConsent;
    use crate::declared_id_token_signing::DeclaredIdTokenSigning;
    use crate::declared_token_exchange::DeclaredTokenExchange;

    const ANCHOR: &str = "crate::Client";
    const ISSUANCE: &str = "use margaret::framework::macros::issues_resource_tokens;\nuse margaret::framework::macros::issues_tokens;\nuse margaret::framework::macros::oauth_scope;\nuse margaret::framework::accepted_clients::client_keys::ClientKeys;\nuse margaret::framework::accepted_clients::consent_policy::ConsentPolicy;\nuse margaret::framework::jose_parameters::jws_algorithm::JwsAlgorithm;\nuse margaret::framework::jwks_secret_store::id_token_signing::IdTokenSigning;\nuse margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod;\nuse margaret::framework::oauth_vocabulary::openid_scope::OpenidScope;\nuse crate::routes::Callback;\n\n#[oauth_scope(name = \"artifacts:read\")]\npub struct ArtifactsReadScope;\n#[oauth_scope(name = \"deploy\")]\npub struct DeployScope;\n#[oauth_scope(name = \"profile\")]\npub struct ProfileScope;\n\nmod routes {\n    pub struct Callback;\n    pub struct Other;\n}\n\n#[issues_tokens(provider, issuer = \"https://issuer.example\")]\npub struct Issuer;\n#[issues_resource_tokens(artifacts, audience = \"artifacts\")]\npub struct Artifacts;\n#[issues_resource_tokens(reports, audience = \"reports\")]\npub struct Reports;\n";
    const PRIVATE_KEY_JWT: &str = "authentication = ClientAuthenticationMethod::PrivateKeyJwt(client_credentials(scopes = [ArtifactsReadScope]), introspection, keys = ClientKeys::Published(jwks_uri = \"https://portal.example/jwks.json\", signing = JwsAlgorithm::Es256))";
    const AUTHORIZATION_CODE: &str = "authorization_code(consent = ConsentPolicy::Prompted, id_token_signing = IdTokenSigning::Rsa, redirect_uris = [\"https://portal.example/callback\"], scopes = [OpenidScope, ProfileScope], refresh_token)";

    fn read<TOutcome>(
        arguments: &str,
        outcome: impl FnOnce(Result<DeclaredAcceptedClients, AcceptedClientsCodegenError>) -> TOutcome,
    ) -> TOutcome {
        let indexed = IndexedSource::new(&format!(
            "use margaret::framework::macros::admits_oauth_client;\n{ISSUANCE}#[admits_oauth_client(admitted_app, {arguments})]\npub struct Client;\n"
        ));
        let issuance = DeclaredTokenIssuance::read(&indexed.index).expect("the issuance is read");

        outcome(DeclaredAcceptedClients::read(
            &indexed.index,
            &issuance,
            &DeclaredResourceIssuances::read(&indexed.index, &issuance)
                .expect("the resources are read"),
            &DeclaredScopes::read(&indexed.index).expect("the scopes are read"),
        ))
    }

    fn declaration_rejection(attribute_arguments: &str) -> AcceptedClientsCodegenError {
        let indexed = IndexedSource::new(&format!(
            "use margaret::framework::macros::admits_oauth_client;\n{ISSUANCE}#[admits_oauth_client({attribute_arguments})]\npub struct Client;\n"
        ));
        let issuance = DeclaredTokenIssuance::read(&indexed.index).expect("the issuance is read");

        DeclaredAcceptedClients::read(
            &indexed.index,
            &issuance,
            &DeclaredResourceIssuances::read(&indexed.index, &issuance)
                .expect("the resources are read"),
            &DeclaredScopes::read(&indexed.index).expect("the scopes are read"),
        )
        .err()
        .expect("the declaration is rejected")
    }

    fn rejection(arguments: &str) -> AcceptedClientsCodegenError {
        read(arguments, |read| {
            read.err().expect("the declaration is rejected")
        })
    }

    #[test]
    fn reads_every_privilege_a_confidential_client_declares_so_the_issuer_enforces_each_one() {
        read(
            &format!(
                "{PRIVATE_KEY_JWT}, {AUTHORIZATION_CODE}, client_id = \"portal\", resources = [artifacts, reports], token_exchange(scopes = [DeployScope])"
            ),
            |read| {
                let accepted = read.expect("the client is accepted");
                let client = accepted.clients.first().expect("the client is accepted");

                assert_eq!(accepted.clients.len(), 1);
                assert!(matches!(
                    &client.authentication,
                    DeclaredAcceptedAuthentication::PrivateKeyJwt(confidential)
                        if confidential.introspection
                            && matches!(
                                &confidential.keys,
                                DeclaredClientKeys::Published { jwks_uri, signing }
                                    if jwks_uri.as_str() == "https://portal.example/jwks.json"
                                        && *signing == JwsAlgorithm::Es256
                            )
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
                assert_eq!(client.resources.admitted().len(), 2);
                assert!(matches!(
                    &client.token_exchange,
                    DeclaredTokenExchange::Granted { scopes }
                        if scopes.iter().map(Scope::as_str).eq(["deploy"])
                ));
                assert_eq!(client.anchor.identifier.field(), "client");
            },
        );
    }

    #[test]
    fn reads_a_bare_public_client_as_withholding_every_grant_so_grants_are_opt_in() {
        read(
            "authentication = ClientAuthenticationMethod::None, client_id = \"spa\", resources = [artifacts]",
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
                assert_eq!(
                    discriminant(&client.token_exchange),
                    discriminant(&DeclaredTokenExchange::Withheld)
                );
                assert!(client.grant_types().is_empty());
                assert!(client.scopes().is_empty());
            },
        );
    }

    #[test]
    fn grants_the_types_and_scopes_it_declares() {
        read(
            &format!(
                "{PRIVATE_KEY_JWT}, {AUTHORIZATION_CODE}, client_id = \"portal\", resources = [artifacts], token_exchange(scopes = [DeployScope])"
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
                    vec!["artifacts:read", "deploy", "openid", "profile"]
                );
            },
        );
    }

    #[test]
    fn reads_a_confidential_client_without_its_optional_privileges() {
        read(
            "authentication = ClientAuthenticationMethod::PrivateKeyJwt(keys = ClientKeys::Published(jwks_uri = \"https://portal.example/jwks.json\", signing = JwsAlgorithm::Es256)), client_id = \"portal\", resources = [artifacts]",
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
    fn rejects_a_client_without_authentication_so_its_method_is_never_defaulted() {
        assert!(matches!(
            rejection(
                "client_id = \"spa\", resources = [artifacts]"
            ),
            AcceptedClientsCodegenError::MissingAuthentication { anchor, .. }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_an_authentication_method_outside_the_vocabulary() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::Password, client_id = \"spa\", resources = [artifacts]"
            ),
            AcceptedClientsCodegenError::UnknownAuthentication { anchor, written }
                if anchor == ANCHOR && written == "ClientAuthenticationMethod :: Password"
        ));
    }

    #[test]
    fn rejects_a_client_authenticating_with_a_shared_secret() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::ClientSecretBasic, client_id = \"spa\", resources = [artifacts]"
            ),
            AcceptedClientsCodegenError::UnsupportedAuthentication { anchor, method }
                if anchor == ANCHOR && method == "client_secret_basic"
        ));
    }

    #[test]
    fn rejects_introspection_for_a_public_client_since_it_cannot_authenticate() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::None(introspection), client_id = \"spa\", resources = [artifacts]"
            ),
            AcceptedClientsCodegenError::AttributeArguments(AttributeArgumentsError::UnrecognizedArgument { argument, .. })
                if argument == "introspection"
        ));
    }

    #[test]
    fn rejects_a_confidential_client_without_its_keys() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::PrivateKeyJwt(introspection), client_id = \"portal\", resources = [artifacts]"
            ),
            AcceptedClientsCodegenError::MissingClientKeys { anchor }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_unknown_client_keys_naming_them_rather_than_guessing_a_verifier() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::PrivateKeyJwt(keys = ClientKeys::Shared), client_id = \"portal\", resources = [artifacts]"
            ),
            AcceptedClientsCodegenError::UnknownClientKeys { anchor, written }
                if anchor == ANCHOR && written == "ClientKeys :: Shared"
        ));
    }

    #[test]
    fn reads_a_confidential_client_verified_by_the_own_keys() {
        read(
            "authentication = ClientAuthenticationMethod::PrivateKeyJwt(keys = ClientKeys::Own), client_id = \"portal\", resources = [artifacts]",
            |read| {
                let accepted = read.expect("the client is accepted");
                let client = accepted.clients.first().expect("the client is accepted");

                assert!(matches!(
                    &client.authentication,
                    DeclaredAcceptedAuthentication::PrivateKeyJwt(confidential)
                        if discriminant(&confidential.keys) == discriminant(&DeclaredClientKeys::Own)
                ));
            },
        );
    }

    #[test]
    fn rejects_published_keys_without_their_jwks_uri() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::PrivateKeyJwt(keys = ClientKeys::Published(signing = JwsAlgorithm::Es256)), client_id = \"portal\", resources = [artifacts]"
            ),
            AcceptedClientsCodegenError::MissingJwksUri { anchor }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_published_keys_without_their_pinned_algorithm() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::PrivateKeyJwt(keys = ClientKeys::Published(jwks_uri = \"https://portal.example/jwks.json\")), client_id = \"portal\", resources = [artifacts]"
            ),
            AcceptedClientsCodegenError::MissingSigningAlgorithm { anchor }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_an_algorithm_that_is_not_fully_specified() {
        assert_eq!(
            rejection(
                "authentication = ClientAuthenticationMethod::PrivateKeyJwt(keys = ClientKeys::Published(jwks_uri = \"https://portal.example/jwks.json\", signing = JwsAlgorithm::EdDsa)), client_id = \"portal\", resources = [artifacts]"
            )
            .to_string(),
            "#[admits_oauth_client] on 'crate::Client' pins its assertions to JwsAlgorithm::EdDsa, which does not name its curve (RFC 9864), so pin JwsAlgorithm::Ed25519 instead"
        );
    }

    #[test]
    fn rejects_an_algorithm_that_is_not_a_variant() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::PrivateKeyJwt(keys = ClientKeys::Published(jwks_uri = \"https://portal.example/jwks.json\", signing = JwsAlgorithm::Hs256)), client_id = \"portal\", resources = [artifacts]"
            ),
            AcceptedClientsCodegenError::UnknownSigningAlgorithm { anchor, written }
                if anchor == ANCHOR && written == "JwsAlgorithm :: Hs256"
        ));
    }

    #[test]
    fn rejects_a_token_exchange_without_its_scopes() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::None, client_id = \"ci\", resources = [artifacts], token_exchange()"
            ),
            AcceptedClientsCodegenError::MissingTokenExchangeScopes { anchor }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_a_token_exchange_scope_listed_twice_since_a_client_holds_each_scope_once() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::None, client_id = \"ci\", resources = [artifacts], token_exchange(scopes = [DeployScope, crate::DeployScope])"
            ),
            AcceptedClientsCodegenError::RepeatedScope { anchor, written }
                if anchor == ANCHOR && written == "crate :: DeployScope"
        ));
    }

    #[test]
    fn rejects_a_token_exchange_scope_that_no_declaration_declares() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::None, client_id = \"ci\", resources = [artifacts], token_exchange(scopes = [UndeclaredScope])"
            ),
            AcceptedClientsCodegenError::UnknownScope { anchor, written }
                if anchor == ANCHOR && written == "UndeclaredScope"
        ));
    }

    #[test]
    fn rejects_a_jwks_uri_that_is_not_a_string() {
        assert!(matches!(
            rejection("authentication = ClientAuthenticationMethod::PrivateKeyJwt(keys = ClientKeys::Published(jwks_uri = jwks, signing = JwsAlgorithm::Es256)), client_id = \"portal\", resources = [artifacts]"),
            AcceptedClientsCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "jwks_uri"
        ));
    }

    #[test]
    fn rejects_a_client_without_its_tag_since_other_declarations_bind_to_it_by_tag() {
        assert!(matches!(
            declaration_rejection("authentication = ClientAuthenticationMethod::None, client_id = \"spa\", resources = [artifacts]"),
            AcceptedClientsCodegenError::MissingTag { anchor } if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_a_client_tagged_by_a_qualified_path() {
        assert!(matches!(
            declaration_rejection("crate::admitted_app, authentication = ClientAuthenticationMethod::None, client_id = \"spa\", resources = [artifacts]"),
            AcceptedClientsCodegenError::MalformedTag { anchor } if anchor == ANCHOR
        ));
    }

    #[test]
    fn pins_the_signing_algorithm_a_client_declares() {
        read(
            "authentication = ClientAuthenticationMethod::PrivateKeyJwt(keys = ClientKeys::Published(jwks_uri = \"https://portal.example/jwks.json\", signing = JwsAlgorithm::Rs256)), client_id = \"portal\", resources = [artifacts]",
            |read| {
                let accepted = read.expect("the client is accepted");

                assert!(matches!(
                    &accepted.clients.first().expect("the client is accepted").authentication,
                    DeclaredAcceptedAuthentication::PrivateKeyJwt(confidential)
                        if matches!(
                            &confidential.keys,
                            DeclaredClientKeys::Published { signing, .. } if *signing == JwsAlgorithm::Rs256
                        )
                ));
            },
        );
    }

    #[test]
    fn rejects_a_signing_algorithm_that_is_not_a_path() {
        assert!(matches!(
            rejection("authentication = ClientAuthenticationMethod::PrivateKeyJwt(keys = ClientKeys::Published(jwks_uri = \"https://portal.example/jwks.json\", signing = \"Es256\")), client_id = \"portal\", resources = [artifacts]"),
            AcceptedClientsCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "signing"
        ));
    }

    #[test]
    fn rejects_redirect_routes_that_are_not_a_list_of_paths() {
        assert!(matches!(
            rejection("authentication = ClientAuthenticationMethod::None, authorization_code(consent = ConsentPolicy::Implicit, id_token_signing = IdTokenSigning::Rsa, redirect_routes = \"callback\", scopes = []), client_id = \"spa\", resources = [artifacts]"),
            AcceptedClientsCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "redirect_routes"
        ));
    }

    #[test]
    fn rejects_a_consent_that_is_not_a_path() {
        assert!(matches!(
            rejection("authentication = ClientAuthenticationMethod::None, authorization_code(consent = \"prompted\", id_token_signing = IdTokenSigning::Rsa, redirect_uris = [\"https://spa.example/callback\"], scopes = []), client_id = \"spa\", resources = [artifacts]"),
            AcceptedClientsCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "consent"
        ));
    }

    #[test]
    fn rejects_an_id_token_signing_that_is_not_a_path() {
        assert!(matches!(
            rejection("authentication = ClientAuthenticationMethod::None, authorization_code(consent = ConsentPolicy::Implicit, id_token_signing = \"rsa\", redirect_uris = [\"https://spa.example/callback\"], scopes = []), client_id = \"spa\", resources = [artifacts]"),
            AcceptedClientsCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "id_token_signing"
        ));
    }

    #[test]
    fn rejects_token_exchange_scopes_that_are_not_an_array() {
        assert!(matches!(
            rejection("authentication = ClientAuthenticationMethod::None, client_id = \"spa\", resources = [artifacts], token_exchange(scopes = \"deploy\")"),
            AcceptedClientsCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "scopes"
        ));
    }

    #[test]
    fn rejects_a_jwks_uri_that_is_not_https() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::PrivateKeyJwt(keys = ClientKeys::Published(jwks_uri = \"http://portal.example/jwks.json\", signing = JwsAlgorithm::Es256)), client_id = \"portal\", resources = [artifacts]"
            ),
            AcceptedClientsCodegenError::MalformedJwksUri { anchor, .. }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_client_credentials_without_scopes_so_their_reach_is_always_explicit() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::PrivateKeyJwt(client_credentials(), keys = ClientKeys::Published(jwks_uri = \"https://portal.example/jwks.json\", signing = JwsAlgorithm::Es256)), client_id = \"portal\", resources = [artifacts]"
            ),
            AcceptedClientsCodegenError::MissingClientCredentialsScopes { anchor, .. }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_client_credentials_scopes_that_are_not_an_array() {
        assert!(matches!(
            rejection("authentication = ClientAuthenticationMethod::PrivateKeyJwt(client_credentials(scopes = \"profile\"), keys = ClientKeys::Published(jwks_uri = \"https://portal.example/jwks.json\", signing = JwsAlgorithm::Es256)), client_id = \"portal\", resources = [artifacts]"),
            AcceptedClientsCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "scopes"
        ));
    }

    #[test]
    fn rejects_openid_as_a_client_credentials_scope() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::PrivateKeyJwt(client_credentials(scopes = [OpenidScope]), keys = ClientKeys::Published(jwks_uri = \"https://portal.example/jwks.json\", signing = JwsAlgorithm::Es256)), client_id = \"portal\", resources = [artifacts]"
            ),
            AcceptedClientsCodegenError::MalformedClientCredentialsScope { anchor, .. }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_an_authorization_code_grant_without_its_consent() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::None, authorization_code(id_token_signing = IdTokenSigning::Rsa, redirect_uris = [\"https://spa.example/callback\"], scopes = []), client_id = \"spa\", resources = [artifacts]"
            ),
            AcceptedClientsCodegenError::MissingConsent { anchor, .. }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_an_unknown_consent_policy_naming_it_rather_than_guessing_one() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::None, authorization_code(consent = ConsentPolicy::Never, id_token_signing = IdTokenSigning::Rsa, redirect_uris = [\"https://spa.example/callback\"], scopes = []), client_id = \"spa\", resources = [artifacts]"
            ),
            AcceptedClientsCodegenError::UnknownConsent { anchor, written }
                if anchor == ANCHOR && written == "ConsentPolicy :: Never"
        ));
    }

    #[test]
    fn rejects_an_authorization_code_grant_without_its_id_token_signing() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::None, authorization_code(consent = ConsentPolicy::Implicit, redirect_uris = [\"https://spa.example/callback\"], scopes = []), client_id = \"spa\", resources = [artifacts]"
            ),
            AcceptedClientsCodegenError::MissingIdTokenSigning { anchor, .. }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_an_id_token_signing_outside_the_grammar() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::None, authorization_code(consent = ConsentPolicy::Implicit, id_token_signing = IdTokenSigning::Hmac, redirect_uris = [\"https://spa.example/callback\"], scopes = []), client_id = \"spa\", resources = [artifacts]"
            ),
            AcceptedClientsCodegenError::UnknownIdTokenSigning { anchor, written }
                if anchor == ANCHOR && written == "IdTokenSigning :: Hmac"
        ));
    }

    fn code_grant_redirecting(redirects: &str) -> String {
        format!(
            "authentication = ClientAuthenticationMethod::None, authorization_code(consent = ConsentPolicy::Implicit, id_token_signing = IdTokenSigning::Rsa, {redirects}, scopes = [OpenidScope]), client_id = \"spa\", resources = [artifacts]"
        )
    }

    #[test]
    fn reads_the_redirect_routes_of_an_authorization_code_grant() {
        read(
            &code_grant_redirecting("redirect_routes = [Callback, routes::Other]"),
            |read| {
                assert!(matches!(
                    read.as_ref().map(|accepted| accepted.clients.as_slice()),
                    Ok([client]) if matches!(
                        &client.authorization_code,
                        DeclaredCodeGrant::Granted(policy)
                            if policy.redirect_uris.is_empty()
                                && policy
                                    .redirect_routes
                                    .iter()
                                    .map(ToString::to_string)
                                    .eq(["crate::routes::Callback", "crate::routes::Other"])
                    )
                ));
            },
        );
    }

    #[test]
    fn rejects_an_empty_list_of_redirect_routes() {
        assert_eq!(
            rejection(&code_grant_redirecting("redirect_routes = []")).to_string(),
            "#[admits_oauth_client] on 'crate::Client' declares an empty list of redirect_routes"
        );
    }

    #[test]
    fn rejects_a_redirect_route_that_names_no_item() {
        assert_eq!(
            rejection(&code_grant_redirecting("redirect_routes = [Missing]")).to_string(),
            "#[admits_oauth_client] on 'crate::Client' redirects to 'Missing', which names no item"
        );
    }

    #[test]
    fn rejects_a_redirect_route_listed_twice_under_different_paths_as_ambiguous() {
        assert_eq!(
            rejection(&code_grant_redirecting(
                "redirect_routes = [Callback, routes::Callback]"
            ))
            .to_string(),
            "#[admits_oauth_client] on 'crate::Client' redirects to the route 'crate::routes::Callback' more than once"
        );
    }

    #[test]
    fn rejects_an_authorization_code_grant_without_its_redirect_uris() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::None, authorization_code(consent = ConsentPolicy::Implicit, id_token_signing = IdTokenSigning::EllipticCurve, scopes = []), client_id = \"spa\", resources = [artifacts]"
            ),
            AcceptedClientsCodegenError::MissingRedirectUris { anchor, .. }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_redirect_uris_that_are_not_an_array() {
        assert!(matches!(
            rejection("authentication = ClientAuthenticationMethod::None, authorization_code(consent = ConsentPolicy::Implicit, id_token_signing = IdTokenSigning::EllipticCurve, redirect_uris = \"https://spa.example/callback\", scopes = []), client_id = \"spa\", resources = [artifacts]"),
            AcceptedClientsCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "redirect_uris"
        ));
    }

    #[test]
    fn rejects_an_empty_list_of_redirect_uris() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::None, authorization_code(consent = ConsentPolicy::Implicit, id_token_signing = IdTokenSigning::EllipticCurve, redirect_uris = [], scopes = []), client_id = \"spa\", resources = [artifacts]"
            ),
            AcceptedClientsCodegenError::EmptyRedirectUris { anchor, .. }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_an_insecure_redirect_uri_so_codes_never_travel_unencrypted() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::None, authorization_code(consent = ConsentPolicy::Implicit, id_token_signing = IdTokenSigning::EllipticCurve, redirect_uris = [\"http://spa.example/callback\"], scopes = []), client_id = \"spa\", resources = [artifacts]"
            ),
            AcceptedClientsCodegenError::InsecureRedirectUri { anchor, redirect_uri }
                if anchor == ANCHOR && redirect_uri == "http://spa.example/callback"
        ));
    }

    #[test]
    fn rejects_an_authorization_code_grant_without_its_scopes() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::None, authorization_code(consent = ConsentPolicy::Implicit, id_token_signing = IdTokenSigning::EllipticCurve, redirect_uris = [\"https://spa.example/callback\"]), client_id = \"spa\", resources = [artifacts]"
            ),
            AcceptedClientsCodegenError::MissingCodeScopes { anchor, .. }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_authorization_code_scopes_that_are_not_an_array() {
        assert!(matches!(
            rejection("authentication = ClientAuthenticationMethod::None, authorization_code(consent = ConsentPolicy::Implicit, id_token_signing = IdTokenSigning::EllipticCurve, redirect_uris = [\"https://spa.example/callback\"], scopes = \"openid\"), client_id = \"spa\", resources = [artifacts]"),
            AcceptedClientsCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "scopes"
        ));
    }

    #[test]
    fn rejects_an_authorization_code_scope_written_as_a_string() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::None, authorization_code(consent = ConsentPolicy::Implicit, id_token_signing = IdTokenSigning::EllipticCurve, redirect_uris = [\"https://spa.example/callback\"], scopes = [\"profile\"]), client_id = \"spa\", resources = [artifacts]"
            ),
            AcceptedClientsCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "scopes"
        ));
    }

    #[test]
    fn rejects_a_client_without_a_client_id() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::None, resources = [artifacts]"
            ),
            AcceptedClientsCodegenError::MissingClientId { anchor, .. }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_a_client_id_that_is_not_a_string() {
        assert!(matches!(
            rejection("authentication = ClientAuthenticationMethod::None, client_id = spa, resources = [artifacts]"),
            AcceptedClientsCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "client_id"
        ));
    }

    #[test]
    fn rejects_a_client_id_with_invisible_characters() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::None, client_id = \"s\\tpa\", resources = [artifacts]"
            ),
            AcceptedClientsCodegenError::MalformedClientId { anchor, .. }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_a_client_identified_by_a_uuid() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::None, client_id = \"67e55044-10b1-426f-9247-bb680e5fe0c8\", resources = [artifacts]"
            ),
            AcceptedClientsCodegenError::UuidClientId { anchor, client_id }
                if anchor == ANCHOR && client_id == "67e55044-10b1-426f-9247-bb680e5fe0c8"
        ));
    }

    #[test]
    fn rejects_a_client_identified_by_the_issuer() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::None, client_id = \"https://issuer.example\", resources = [artifacts]"
            ),
            AcceptedClientsCodegenError::IssuerClientId { anchor, client_id }
                if anchor == ANCHOR && client_id == "https://issuer.example"
        ));
    }

    #[test]
    fn rejects_a_client_without_resources_so_its_token_audiences_are_explicit() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::None, client_id = \"spa\""
            ),
            AcceptedClientsCodegenError::MissingResources { anchor, .. }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_resources_that_are_not_an_array() {
        assert!(matches!(
            rejection("authentication = ClientAuthenticationMethod::None, client_id = \"spa\", resources = \"artifacts\""),
            AcceptedClientsCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "resources"
        ));
    }

    #[test]
    fn reads_a_sign_in_client_declaring_no_resources_so_it_signs_in_without_a_resource_audience() {
        read(
            "authentication = ClientAuthenticationMethod::None, authorization_code(consent = ConsentPolicy::Implicit, id_token_signing = IdTokenSigning::Rsa, redirect_uris = [\"https://staff.example/callback\"], scopes = [OpenidScope]), client_id = \"staff\", resources = []",
            |read| {
                let accepted = read.expect("the client is accepted");
                let client = accepted.clients.first().expect("the client is accepted");

                assert_eq!(
                    discriminant(&client.resources),
                    discriminant(&ClientResources::Withheld)
                );
                assert!(matches!(
                    &client.authorization_code,
                    DeclaredCodeGrant::Granted(policy)
                        if policy.scopes.iter().map(Scope::as_str).eq(["openid"])
                ));
            },
        );
    }

    #[test]
    fn rejects_a_client_declaring_no_resources_without_a_code_grant_since_it_could_obtain_no_token()
    {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::None, client_id = \"spa\", resources = []"
            ),
            AcceptedClientsCodegenError::ResourcelessClientWithoutCodeGrant { anchor }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_a_code_grant_without_openid_of_a_client_declaring_no_resources_since_its_tokens_could_reach_no_audience()
     {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::None, authorization_code(consent = ConsentPolicy::Implicit, id_token_signing = IdTokenSigning::Rsa, redirect_uris = [\"https://staff.example/callback\"], scopes = [ProfileScope]), client_id = \"staff\", resources = []"
            ),
            AcceptedClientsCodegenError::ResourcelessCodeGrantWithoutOpenidScope { anchor }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_client_credentials_of_a_client_declaring_no_resources_since_their_tokens_could_reach_no_audience()
     {
        assert!(matches!(
            rejection(&format!(
                "authentication = ClientAuthenticationMethod::PrivateKeyJwt(client_credentials(scopes = [ArtifactsReadScope]), keys = ClientKeys::Own), {AUTHORIZATION_CODE}, client_id = \"portal\", resources = []"
            )),
            AcceptedClientsCodegenError::ClientCredentialsWithoutResources { anchor }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_introspection_by_a_client_declaring_no_resources_since_no_token_could_be_addressed_to_it()
     {
        assert!(matches!(
            rejection(&format!(
                "authentication = ClientAuthenticationMethod::PrivateKeyJwt(introspection, keys = ClientKeys::Own), {AUTHORIZATION_CODE}, client_id = \"portal\", resources = []"
            )),
            AcceptedClientsCodegenError::IntrospectionWithoutResources { anchor }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_token_exchange_by_a_client_declaring_no_resources_since_exchanged_tokens_could_reach_no_audience()
     {
        assert!(matches!(
            rejection(&format!(
                "authentication = ClientAuthenticationMethod::None, {AUTHORIZATION_CODE}, client_id = \"ci\", resources = [], token_exchange(scopes = [DeployScope])"
            )),
            AcceptedClientsCodegenError::TokenExchangeWithoutResources { anchor }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_a_resource_that_is_not_a_plain_tag() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::None, client_id = \"spa\", resources = [files::artifacts]"
            ),
            AcceptedClientsCodegenError::MalformedResourceTag { anchor }
                if anchor == ANCHOR
        ));
    }

    #[test]
    fn rejects_a_resource_no_declaration_issues_tokens_for() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::None, client_id = \"spa\", resources = [session]"
            ),
            AcceptedClientsCodegenError::UnknownResource { anchor, tag }
                if anchor == ANCHOR && tag == "session"
        ));
    }

    #[test]
    fn rejects_a_resource_named_twice_rather_than_silently_collapsing_the_duplicate() {
        assert!(matches!(
            rejection(
                "authentication = ClientAuthenticationMethod::None, client_id = \"spa\", resources = [artifacts, artifacts]"
            ),
            AcceptedClientsCodegenError::DuplicateResource { anchor, tag }
                if anchor == ANCHOR && tag == "artifacts"
        ));
    }

    #[test]
    fn reports_unparseable_client_arguments_naming_the_attribute_so_the_author_can_locate_them() {
        assert!(matches!(
            rejection(
                "= 5"
            ),
            AcceptedClientsCodegenError::Index(AttributeError::Arguments(AttributeArgumentsError::Malformed { attribute_path, .. }))
                if attribute_path == "admits_oauth_client"
        ));
    }
}
