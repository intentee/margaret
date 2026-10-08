use std::collections::HashMap;
use std::ptr;

use margaret_accepted_clients_codegen::accepted_client_declaration::AcceptedClientDeclaration;
use margaret_accepted_clients_codegen::declared_accepted_authentication::DeclaredAcceptedAuthentication;
use margaret_accepted_clients_codegen::declared_accepted_clients::DeclaredAcceptedClients;
use margaret_accepted_clients_codegen::declared_client_keys::DeclaredClientKeys;
use margaret_accepted_clients_codegen::declared_code_grant::DeclaredCodeGrant;
use margaret_accepted_clients_codegen::declared_confidential_client::DeclaredConfidentialClient;
use margaret_accepted_clients_codegen::declared_token_exchange::DeclaredTokenExchange;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_attribute::IndexedAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::tag::Tag;
use margaret_oauth_client_codegen::declared_client_registration::DeclaredClientRegistration;
use margaret_oauth_client_codegen::declared_oauth_clients::DeclaredOAuthClients;
use margaret_oauth_client_codegen::declared_sign_in::DeclaredSignIn;
use margaret_oauth_client_codegen::oauth_client_declaration::OAuthClientDeclaration;
use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_token_issuance_codegen::declared_resource_issuances::DeclaredResourceIssuances;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;
use margaret_trusted_issuer_codegen::declared_trusts::DeclaredTrusts;
use margaret_trusted_issuer_codegen::trusted_issuer_binding::TrustedIssuerBinding;
use margaret_trusted_issuer_codegen::trusted_issuer_group::TrustedIssuerGroup;

use crate::bearer_token_addressee::BearerTokenAddressee;
use crate::bound_authorization_server::BoundAuthorizationServer;
use crate::bound_sign_in::BoundSignIn;
use crate::oauth_client_binding::OAuthClientBinding;
use crate::read_bearer_token_addressee::read_bearer_token_addressee;
use crate::read_middleware_attribute::read_middleware_attribute;
use crate::scanned_bearer_token::ScannedBearerToken;
use crate::subject_token_exchanger_binding::SubjectTokenExchangerBinding;
use crate::tag_error::TagError;
use crate::tag_expectation::TagExpectation;
use crate::tag_kind::TagKind;
use crate::tagged_item::TaggedItem;
use crate::trusted_issuer_kind::TrustedIssuerKind;

struct TagEntry<'index> {
    item: &'index IndexedItem,
    kind: TagKind,
}

struct SortedEntry<'pool, 'index> {
    entry: &'pool TagEntry<'index>,
    tag: &'pool Tag,
}

fn unknown_tag(tag: &Tag, expected: TagExpectation, site: String) -> TagError {
    TagError::UnknownTag {
        site,
        tag: tag.to_string(),
        kind: expected,
    }
}

fn wrong_kind(tag: &Tag, expected: TagExpectation, site: String, entry: &TagEntry) -> TagError {
    TagError::WrongKind {
        site,
        tag: tag.to_string(),
        expected,
        found: entry.kind,
    }
}

fn resolve_entry<'pool, 'index>(
    entries: &'pool HashMap<Tag, TagEntry<'index>>,
    tag: &Tag,
    expected: TagExpectation,
    site: &str,
) -> Result<&'pool TagEntry<'index>, TagError> {
    match entries.get(tag) {
        Some(entry) if expected.admits(entry.kind) => Ok(entry),
        Some(entry) => Err(wrong_kind(tag, expected, site.to_string(), entry)),
        None => Err(unknown_tag(tag, expected, site.to_string())),
    }
}

fn scan_bearer_tokens<'index>(
    index: &'index AttributeIndex,
    entries: &HashMap<Tag, TagEntry<'index>>,
    clients: &DeclaredOAuthClients<'index>,
) -> Result<Vec<ScannedBearerToken<'index>>, TagError> {
    let mut scanned = Vec::new();

    for item in index.items() {
        for method in item.methods() {
            for parameter in method.parameters() {
                let Some(attribute) =
                    parameter.framework_attribute(FrameworkAttribute::BearerToken)
                else {
                    continue;
                };
                let site = format!(
                    "argument #{} of '{}::{}'",
                    parameter.position(),
                    item.canonical_path(),
                    method.identifier()
                );
                let addressee = read_bearer_token_addressee(attribute.args()?, &site)?;

                match &addressee {
                    BearerTokenAddressee::Client(tag) => {
                        resolve_entry(entries, tag, TagExpectation::OAuthClient, &site)?;

                        if clients.clients.iter().any(|client| {
                            client.tag == *tag
                                && matches!(
                                    client.registration,
                                    DeclaredClientRegistration::Admitted { .. }
                                )
                        }) {
                            return Err(TagError::IntrospectionThroughOwnClient {
                                client: tag.to_string(),
                                site,
                            });
                        }
                    }
                    BearerTokenAddressee::Issuer(tag) => {
                        resolve_entry(entries, tag, TagExpectation::TokenIssuer, &site)?;
                    }
                    BearerTokenAddressee::Resource(tag) => {
                        resolve_entry(entries, tag, TagExpectation::ResourceTokens, &site)?;
                    }
                }

                scanned.push(ScannedBearerToken {
                    addressee,
                    attribute,
                });
            }
        }
    }

    Ok(scanned)
}

fn trusted_issuer_kind(group: &TrustedIssuerGroup) -> TrustedIssuerKind {
    match group {
        TrustedIssuerGroup::Discovered { .. } => TrustedIssuerKind::OidcIssuer,
        TrustedIssuerGroup::JwksEndpoint { .. } => TrustedIssuerKind::JwksEndpoint,
    }
}

fn admit<'index>(
    entries: &mut HashMap<Tag, TagEntry<'index>>,
    tag: Tag,
    entry: TagEntry<'index>,
) -> Result<(), TagError> {
    if let Some(existing) = entries.get(&tag) {
        return Err(TagError::DuplicateTag {
            tag: tag.to_string(),
            first: existing.item.canonical_path().to_string(),
            second: entry.item.canonical_path().to_string(),
        });
    }

    entries.insert(tag, entry);

    Ok(())
}

fn collect_middleware_handlers<'index>(
    index: &'index AttributeIndex,
    entries: &mut HashMap<Tag, TagEntry<'index>>,
) -> Result<(), TagError> {
    for matched in index.select_framework_attribute(FrameworkAttribute::HandlesMiddlewareAttribute)
    {
        let item = matched.item();
        let concrete = item.canonical_path().to_string();
        let path = read_middleware_attribute(matched.args()?)?.ok_or_else(|| {
            TagError::MissingMiddlewareTag {
                concrete: concrete.clone(),
            }
        })?;
        let tag = Tag::from_path(&path).ok_or(TagError::MalformedMiddlewareTag { concrete })?;

        admit(
            entries,
            tag,
            TagEntry {
                item,
                kind: TagKind::Middleware,
            },
        )?;
    }

    Ok(())
}

fn external_sign_in(sign_in: &DeclaredSignIn) -> BoundSignIn<'_> {
    match sign_in {
        DeclaredSignIn::Declared {
            redirect_route,
            scopes,
        } => BoundSignIn::Available {
            redirect_route,
            scopes,
        },
        DeclaredSignIn::Undeclared => BoundSignIn::Unavailable,
    }
}

fn own_sign_in<'declarations>(
    admitted: &'declarations AcceptedClientDeclaration,
    site: String,
) -> Result<BoundSignIn<'declarations>, TagError> {
    let DeclaredCodeGrant::Granted(policy) = &admitted.authorization_code else {
        return Ok(BoundSignIn::Unavailable);
    };

    match policy.redirect_routes.as_slice() {
        [] => Ok(BoundSignIn::Unavailable),
        [redirect_route] => Ok(BoundSignIn::Available {
            redirect_route,
            scopes: &policy.scopes,
        }),
        [..] => Err(TagError::AmbiguousOwnRedirectRoute {
            admitted: admitted.tag.to_string(),
            site,
        }),
    }
}

pub struct TagPool<'index> {
    bearer_tokens: Vec<ScannedBearerToken<'index>>,
    entries: HashMap<Tag, TagEntry<'index>>,
}

impl<'index> TagPool<'index> {
    /// # Errors
    ///
    /// Returns `TagError` propagated from the work it performs.
    pub fn collect(
        index: &'index AttributeIndex,
        trusts: &DeclaredTrusts<'index>,
        clients: &DeclaredOAuthClients<'index>,
        issuance: &DeclaredTokenIssuance<'index>,
        resources: &DeclaredResourceIssuances<'index>,
        admitted: &DeclaredAcceptedClients<'index>,
    ) -> Result<Self, TagError> {
        let mut entries: HashMap<Tag, TagEntry<'index>> = HashMap::new();

        for TrustedIssuerBinding { group, trust } in trusts.bindings() {
            admit(
                &mut entries,
                trust.tag.clone(),
                TagEntry {
                    item: trust.anchor,
                    kind: TagKind::TrustedIssuer(trusted_issuer_kind(group)),
                },
            )?;
        }

        collect_middleware_handlers(index, &mut entries)?;

        for client in &clients.clients {
            admit(
                &mut entries,
                client.tag.clone(),
                TagEntry {
                    item: client.anchor,
                    kind: TagKind::OAuthClient,
                },
            )?;
        }

        if let DeclaredTokenIssuance::Declared(declared) = issuance {
            admit(
                &mut entries,
                declared.tag.clone(),
                TagEntry {
                    item: declared.anchor,
                    kind: TagKind::TokenIssuance,
                },
            )?;
        }

        for resource in resources.resources() {
            admit(
                &mut entries,
                resource.tag.clone(),
                TagEntry {
                    item: resource.anchor,
                    kind: TagKind::ResourceTokens,
                },
            )?;
        }

        for client in &admitted.clients {
            admit(
                &mut entries,
                client.tag.clone(),
                TagEntry {
                    item: client.anchor.item,
                    kind: TagKind::AdmittedClient,
                },
            )?;
        }

        let bearer_tokens = scan_bearer_tokens(index, &entries, clients)?;

        if let Some(unconsumed) = resources.resources().find(|resource| {
            !admitted
                .clients
                .iter()
                .any(|client| client.resources.contains(&resource.audience))
                && !bearer_tokens.iter().any(|scanned| {
                    matches!(&scanned.addressee, BearerTokenAddressee::Resource(tag) if *tag == resource.tag)
                })
        }) {
            return Err(TagError::UnconsumedResourceIssuance {
                anchor: unconsumed.anchor.canonical_path().to_string(),
                tag: unconsumed.tag.to_string(),
            });
        }

        Ok(Self {
            bearer_tokens,
            entries,
        })
    }

    #[must_use]
    pub fn bearer_token(&self, attributes: &[IndexedAttribute]) -> Option<&BearerTokenAddressee> {
        self.bearer_tokens
            .iter()
            .find(|scanned| {
                attributes
                    .iter()
                    .any(|attribute| ptr::eq(attribute, scanned.attribute))
            })
            .map(|scanned| &scanned.addressee)
    }

    #[must_use]
    pub fn verifies_bearer_tokens_for(&self, tag: &Tag) -> bool {
        self.bearer_tokens.iter().any(|scanned| {
            matches!(
                &scanned.addressee,
                BearerTokenAddressee::Issuer(verified) | BearerTokenAddressee::Resource(verified)
                    if verified == tag
            )
        })
    }

    #[must_use]
    pub fn middleware_handlers(&self) -> Vec<TaggedItem<'index>> {
        let mut handlers: Vec<TaggedItem<'index>> = self
            .sorted_entries()
            .into_iter()
            .filter(|sorted| TagExpectation::Middleware.admits(sorted.entry.kind))
            .map(|SortedEntry { entry, tag }| TaggedItem {
                item: entry.item,
                kind: entry.kind,
                tag: tag.clone(),
            })
            .collect();

        handlers.sort_by(|left, right| left.item.canonical_path().cmp(right.item.canonical_path()));

        handlers
    }

    /// # Errors
    ///
    /// Returns `TagError::UnknownTag` or `TagError::WrongKind` for an issuer that is not a trusted
    /// issuer or an admitted client that the provider does not admit,
    /// `TagError::OAuthClientIssuerNotDiscovered` for an issuer without discovery,
    /// `TagError::DuplicateOAuthClient` for two clients of one issuer with one client identifier,
    /// `TagError::OwnClientOfPublishedKeys` for an admitted client that verifies other keys, and
    /// `TagError::DuplicateOwnClient` for two clients acting as one admitted client.
    pub fn oauth_client_bindings<'declarations>(
        &self,
        trusts: &'declarations DeclaredTrusts<'index>,
        clients: &'declarations DeclaredOAuthClients<'index>,
        issuance: &'declarations DeclaredTokenIssuance<'index>,
        admitted: &'declarations DeclaredAcceptedClients<'index>,
    ) -> Result<Vec<OAuthClientBinding<'declarations, 'index>>, TagError> {
        let mut bindings: Vec<OAuthClientBinding<'declarations, 'index>> = Vec::new();

        for client in &clients.clients {
            let binding = match &client.registration {
                DeclaredClientRegistration::Admitted { admitted: tag } => {
                    self.own_binding(issuance, admitted, client, tag, &bindings)?
                }
                DeclaredClientRegistration::External {
                    authentication,
                    client_id,
                    issuer,
                    sign_in,
                } => OAuthClientBinding {
                    client,
                    server: BoundAuthorizationServer::External {
                        authentication,
                        client_id,
                        issuer: self
                            .external_issuer(trusts, client, client_id, issuer, &bindings)?,
                    },
                    sign_in: external_sign_in(sign_in),
                },
            };

            bindings.push(binding);
        }

        if let Some(unconsumed) = admitted.clients.iter().find(|client| {
            matches!(
                client.authentication,
                DeclaredAcceptedAuthentication::PrivateKeyJwt(DeclaredConfidentialClient {
                    keys: DeclaredClientKeys::Own,
                    ..
                })
            ) && !bindings.iter().any(|binding| {
                matches!(
                    &binding.server,
                    BoundAuthorizationServer::Own { admitted, .. } if admitted.tag == client.tag
                )
            })
        }) {
            return Err(TagError::UnconsumedOwnKeys {
                anchor: unconsumed.anchor.item.canonical_path().to_string(),
                tag: unconsumed.tag.to_string(),
            });
        }

        Ok(bindings)
    }

    /// # Errors
    ///
    /// Returns `TagError::UnknownTag` or `TagError::WrongKind`.
    pub fn resolve(
        &self,
        tag: &Tag,
        expected: TagExpectation,
        site: &str,
    ) -> Result<&'index IndexedItem, TagError> {
        resolve_entry(&self.entries, tag, expected, site).map(|entry| entry.item)
    }

    /// # Errors
    ///
    /// Returns `TagError::MalformedSubjectTokenExchangerIssuer` for an exchanger that names no
    /// issuer tag, `TagError::UnknownTag` or `TagError::WrongKind` for an issuer that is not a
    /// trusted issuer, and `TagError::DuplicateSubjectTokenExchanger` for a second exchanger of
    /// the same issuer.
    pub fn subject_token_exchanger_bindings<'trusts>(
        &self,
        index: &AttributeIndex,
        trusts: &'trusts DeclaredTrusts<'index>,
        admitted: &DeclaredAcceptedClients<'index>,
    ) -> Result<Vec<SubjectTokenExchangerBinding<'trusts, 'index>>, TagError> {
        let mut bindings: Vec<SubjectTokenExchangerBinding<'trusts, 'index>> = Vec::new();

        for matched in index.select_framework_attribute(FrameworkAttribute::ExchangesTokensFrom) {
            let declaring = matched.item().canonical_path();
            let issuer = matched
                .args()?
                .interpret(|reader| reader.take_path("issuer"))?
                .as_ref()
                .and_then(Tag::from_path)
                .ok_or_else(|| TagError::MalformedSubjectTokenExchangerIssuer {
                    concrete: declaring.to_string(),
                })?;
            let site = format!("the subject token exchanger '{declaring}'");
            let Some(issuer) = trusts.binding(&issuer) else {
                return Err(self.unresolved(&issuer, TagExpectation::TrustedIssuer, site));
            };

            if let Some(existing) = bindings
                .iter()
                .find(|binding| binding.issuer.trust.tag == issuer.trust.tag)
            {
                return Err(TagError::DuplicateSubjectTokenExchanger {
                    issuer: issuer.trust.tag.to_string(),
                    first: existing.declaring.to_string(),
                    second: declaring.to_string(),
                });
            }

            bindings.push(SubjectTokenExchangerBinding {
                declaring: declaring.clone(),
                issuer,
            });
        }

        let exchanging = admitted
            .clients
            .iter()
            .find(|client| matches!(client.token_exchange, DeclaredTokenExchange::Granted { .. }));

        if let Some(binding) = bindings.first()
            && exchanging.is_none()
        {
            return Err(TagError::UnconsumedSubjectTokenExchanger {
                concrete: binding.declaring.to_string(),
            });
        }

        if bindings.is_empty()
            && let Some(client) = exchanging
        {
            return Err(TagError::MissingSubjectTokenExchanger {
                client: client.anchor.item.canonical_path().to_string(),
            });
        }

        Ok(bindings)
    }

    fn external_issuer<'declarations>(
        &self,
        trusts: &'declarations DeclaredTrusts<'index>,
        client: &OAuthClientDeclaration,
        client_id: &ClientId,
        issuer: &Tag,
        bindings: &[OAuthClientBinding<'declarations, 'index>],
    ) -> Result<TrustedIssuerBinding<'declarations, 'index>, TagError> {
        let site = format!("the oauth client '{}'", client.anchor.canonical_path());
        let issuer = match trusts.binding(issuer) {
            Some(binding) if matches!(binding.group, TrustedIssuerGroup::Discovered { .. }) => {
                binding
            }
            Some(binding) => {
                return Err(TagError::OAuthClientIssuerNotDiscovered {
                    issuer: binding.trust.tag.to_string(),
                    site,
                });
            }
            None => return Err(self.unresolved(issuer, TagExpectation::TrustedIssuer, site)),
        };

        match bindings.iter().find(|existing| {
            matches!(
                &existing.server,
                BoundAuthorizationServer::External { client_id: existing_id, issuer: existing_issuer, .. }
                    if *existing_id == client_id
                        && existing_issuer.group.issuer().as_str() == issuer.group.issuer().as_str()
            )
        }) {
            Some(existing) => Err(TagError::DuplicateOAuthClient {
                client_id: client_id.to_string(),
                first: existing.client.anchor.canonical_path().to_string(),
                issuer: issuer.group.issuer().to_string(),
                second: client.anchor.canonical_path().to_string(),
            }),
            None => Ok(issuer),
        }
    }

    fn own_binding<'declarations>(
        &self,
        issuance: &'declarations DeclaredTokenIssuance<'index>,
        admitted: &'declarations DeclaredAcceptedClients<'index>,
        client: &'declarations OAuthClientDeclaration<'index>,
        tag: &Tag,
        bindings: &[OAuthClientBinding<'declarations, 'index>],
    ) -> Result<OAuthClientBinding<'declarations, 'index>, TagError> {
        let site = format!("the oauth client '{}'", client.anchor.canonical_path());
        let DeclaredTokenIssuance::Declared(issuance) = issuance else {
            return Err(self.unresolved(tag, TagExpectation::AdmittedClient, site));
        };
        let Some(found) = admitted
            .clients
            .iter()
            .find(|candidate| candidate.tag == *tag)
        else {
            return Err(self.unresolved(tag, TagExpectation::AdmittedClient, site));
        };

        if !matches!(
            found.authentication,
            DeclaredAcceptedAuthentication::PrivateKeyJwt(DeclaredConfidentialClient {
                keys: DeclaredClientKeys::Own,
                ..
            })
        ) {
            return Err(TagError::OwnClientOfPublishedKeys {
                admitted: tag.to_string(),
                site,
            });
        }

        match bindings.iter().find(|existing| {
            matches!(
                &existing.server,
                BoundAuthorizationServer::Own { admitted: existing_admitted, .. }
                    if existing_admitted.tag == *tag
            )
        }) {
            Some(existing) => Err(TagError::DuplicateOwnClient {
                admitted: tag.to_string(),
                first: existing.client.anchor.canonical_path().to_string(),
                second: client.anchor.canonical_path().to_string(),
            }),
            None => Ok(OAuthClientBinding {
                client,
                server: BoundAuthorizationServer::Own {
                    admitted: found,
                    issuance,
                },
                sign_in: own_sign_in(found, site)?,
            }),
        }
    }

    fn sorted_entries(&self) -> Vec<SortedEntry<'_, 'index>> {
        let mut sorted: Vec<SortedEntry<'_, 'index>> = self
            .entries
            .iter()
            .map(|(tag, entry)| SortedEntry { entry, tag })
            .collect();

        sorted.sort_by_key(|sorted_entry| sorted_entry.tag.to_string());

        sorted
    }

    fn unresolved(&self, tag: &Tag, expected: TagExpectation, site: String) -> TagError {
        match self.entries.get(tag) {
            Some(entry) => wrong_kind(tag, expected, site, entry),
            None => unknown_tag(tag, expected, site),
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::Path;
    use syn::parse_str;

    use margaret_accepted_clients_codegen::declared_accepted_clients::DeclaredAcceptedClients;
    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_attributes::attribute_error::AttributeError;
    use margaret_attributes::indexed_method::IndexedMethod;
    use margaret_attributes::indexed_parameter::IndexedParameter;
    use margaret_attributes::tag::Tag;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_oauth_client_codegen::declared_oauth_clients::DeclaredOAuthClients;
    use margaret_oauth_vocabulary::scope::Scope;
    use margaret_token_issuance_codegen::declared_resource_issuances::DeclaredResourceIssuances;
    use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;
    use margaret_trusted_issuer_codegen::declared_trusts::DeclaredTrusts;

    use crate::bearer_token_addressee::BearerTokenAddressee;
    use crate::bound_authorization_server::BoundAuthorizationServer;
    use crate::bound_sign_in::BoundSignIn;
    use crate::oauth_client_binding::OAuthClientBinding;
    use crate::tag_error::TagError;
    use crate::tag_expectation::TagExpectation;
    use crate::tag_kind::TagKind;
    use crate::tag_pool::TagPool;
    use crate::trusted_issuer_kind::TrustedIssuerKind;

    const PARTNER_TRUST: &str = "#[verifies_tokens_from_issuer(partner, audience = \"api\", issuer = \"https://partner.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Discovered)]\nstruct Issuer;\n";

    const JWKS_TRUST: &str = "#[verifies_tokens_from_issuer(jwks, audience = \"api\", issuer = \"https://jwks.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Published(jwks_uri = \"https://jwks.example/keys\"))]\nstruct JwksEndpoint;\n";

    fn tag(name: &str) -> Tag {
        let path: Path = parse_str(name).expect("the tag path parses");

        Tag::from_path(&path).expect("the tag is a plain name")
    }

    fn trusts(indexed: &IndexedSource) -> DeclaredTrusts<'_> {
        DeclaredTrusts::read(&indexed.index).expect("the trusts are read")
    }

    fn clients(indexed: &IndexedSource) -> DeclaredOAuthClients<'_> {
        DeclaredOAuthClients::read(&indexed.index).expect("the oauth clients are read")
    }

    fn collected(indexed: &IndexedSource) -> Result<TagPool<'_>, TagError> {
        let issuance =
            DeclaredTokenIssuance::read(&indexed.index).expect("the token issuance is read");
        let resources = DeclaredResourceIssuances::read(&indexed.index, &issuance)
            .expect("the resources are read");

        TagPool::collect(
            &indexed.index,
            &trusts(indexed),
            &clients(indexed),
            &issuance,
            &resources,
            &DeclaredAcceptedClients::read(&indexed.index, &issuance, &resources)
                .expect("the admitted clients are read"),
        )
    }

    fn pool(indexed: &IndexedSource) -> TagPool<'_> {
        collected(indexed).expect("the pool collects")
    }

    fn rejection_for(lib_source: &str) -> TagError {
        collected(&IndexedSource::new(lib_source))
            .err()
            .expect("the pool fails to collect")
    }

    fn error_for(lib_source: &str) -> String {
        rejection_for(lib_source).to_string()
    }

    fn oauth_client(tag: &str, client_id: &str, issuer: &str, anchor: &str) -> String {
        format!(
            "#[acts_as_oauth_client({tag}, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::PrivateKeyJwt, client_id = \"{client_id}\", issuer = {issuer})]\nstruct {anchor};\n"
        )
    }

    fn oauth_client_source() -> String {
        format!(
            "{PARTNER_TRUST}\n{}",
            oauth_client("partner_client", "partner", "partner", "PartnerClient")
        )
    }

    fn with_bindings<TOutcome>(
        lib_source: &str,
        inspect: impl FnOnce(Result<Vec<OAuthClientBinding<'_, '_>>, TagError>) -> TOutcome,
    ) -> TOutcome {
        let indexed = IndexedSource::new(lib_source);
        let issuance =
            DeclaredTokenIssuance::read(&indexed.index).expect("the token issuance is read");
        let resources = DeclaredResourceIssuances::read(&indexed.index, &issuance)
            .expect("the resources are read");
        let admitted = DeclaredAcceptedClients::read(&indexed.index, &issuance, &resources)
            .expect("the admitted clients are read");
        let trusts = trusts(&indexed);
        let clients = clients(&indexed);
        let pool = pool(&indexed);

        inspect(pool.oauth_client_bindings(&trusts, &clients, &issuance, &admitted))
    }

    fn oauth_client_rejection(lib_source: &str) -> TagError {
        with_bindings(lib_source, |bindings| {
            bindings.err().expect("the oauth client is rejected")
        })
    }

    fn own_provider_admitting(keys: &str) -> String {
        format!(
            "#[issues_tokens(provider, audience = \"session\", issuer = \"https://issuer.example\")]\nstruct Issuer;\n#[issues_resource_tokens(artifacts, audience = \"artifacts\")]\nstruct Artifacts;\n#[admits_oauth_client(portal_app, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::PrivateKeyJwt(keys = margaret::framework::accepted_clients::client_keys::ClientKeys::{keys}), client_id = \"portal\", resources = [artifacts])]\nstruct Portal;\n"
        )
    }

    fn own_provider_granting(redirects: &str) -> String {
        format!(
            "mod routes {{\n    pub struct Callback;\n    pub struct Landing;\n}}\n#[issues_tokens(provider, audience = \"session\", issuer = \"https://issuer.example\")]\nstruct Issuer;\n#[issues_resource_tokens(artifacts, audience = \"artifacts\")]\nstruct Artifacts;\n#[admits_oauth_client(portal_app, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::PrivateKeyJwt(keys = margaret::framework::accepted_clients::client_keys::ClientKeys::Own), authorization_code(consent = margaret::framework::accepted_clients::consent_policy::ConsentPolicy::Implicit, id_token_signing = margaret::framework::jwks_secret_store::id_token_signing::IdTokenSigning::Rsa, {redirects}, scopes = [\"openid\", \"profile\"]), client_id = \"portal\", resources = [artifacts])]\nstruct Portal;\n{}",
            own_client("portal", "portal_app", "PortalClient")
        )
    }

    fn own_client(tag: &str, admitted: &str, anchor: &str) -> String {
        format!("#[acts_as_oauth_client({tag}, admitted_as = {admitted})]\nstruct {anchor};\n")
    }

    #[test]
    fn binds_an_oauth_client_to_its_discovered_issuer() {
        assert!(with_bindings(&oauth_client_source(), |bindings| matches!(
            bindings.as_deref(),
            Ok([binding]) if binding.client.anchor.canonical_path().to_string() == "crate::PartnerClient"
                && binding.client_id().as_str() == "partner"
                && matches!(
                    &binding.server,
                    BoundAuthorizationServer::External { issuer, .. } if issuer.trust.tag.to_string() == "partner"
                )
        )));
    }

    #[test]
    fn binds_an_oauth_client_of_the_own_provider_to_its_admitted_client() {
        assert!(with_bindings(
            &format!(
                "{}{}",
                own_provider_admitting("Own"),
                own_client("portal", "portal_app", "PortalClient")
            ),
            |bindings| matches!(
                bindings.as_deref(),
                Ok([binding]) if binding.client_id().as_str() == "portal"
                    && matches!(
                        &binding.server,
                        BoundAuthorizationServer::Own { admitted, issuance }
                            if admitted.tag.to_string() == "portal_app"
                                && issuance.tag.to_string() == "provider"
                    )
            )
        ));
    }

    #[test]
    fn binds_the_sign_in_an_external_client_declares() {
        assert!(with_bindings(
            &format!(
                "{PARTNER_TRUST}struct Callback;\n#[acts_as_oauth_client(partner_client, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::PrivateKeyJwt, client_id = \"partner\", issuer = partner, sign_in(redirect_route = Callback, scopes = [\"profile\"]))]\nstruct PartnerClient;\n"
            ),
            |bindings| matches!(
                bindings.as_deref(),
                Ok([binding]) if matches!(
                    binding.sign_in,
                    BoundSignIn::Available { redirect_route, scopes }
                        if redirect_route.to_string() == "crate::Callback"
                            && scopes.iter().map(Scope::as_str).eq(["profile"])
                )
            )
        ));
    }

    #[test]
    fn binds_no_sign_in_for_an_external_client_that_declares_none() {
        assert!(with_bindings(&oauth_client_source(), |bindings| {
            bindings
                .expect("the client binds")
                .into_iter()
                .map(|binding| binding.sign_in)
                .eq([BoundSignIn::Unavailable])
        }));
    }

    #[test]
    fn signs_an_own_client_in_through_the_redirect_route_of_its_admitted_client() {
        assert!(with_bindings(
            &own_provider_granting("redirect_routes = [routes::Callback]"),
            |bindings| matches!(
                bindings.as_deref(),
                Ok([binding]) if matches!(
                    binding.sign_in,
                    BoundSignIn::Available { redirect_route, scopes }
                        if redirect_route.to_string() == "crate::routes::Callback"
                            && scopes.iter().map(Scope::as_str).eq(["openid", "profile"])
                )
            )
        ));
    }

    #[test]
    fn binds_no_sign_in_for_an_own_client_whose_admitted_client_redirects_to_no_route() {
        assert!(with_bindings(
            &own_provider_granting("redirect_uris = [\"https://portal.example/callback\"]"),
            |bindings| {
                bindings
                    .expect("the client binds")
                    .into_iter()
                    .map(|binding| binding.sign_in)
                    .eq([BoundSignIn::Unavailable])
            }
        ));
    }

    #[test]
    fn binds_no_sign_in_for_an_own_client_whose_admitted_client_grants_no_code() {
        assert!(with_bindings(
            &format!(
                "{}{}",
                own_provider_admitting("Own"),
                own_client("portal", "portal_app", "PortalClient")
            ),
            |bindings| {
                bindings
                    .expect("the client binds")
                    .into_iter()
                    .map(|binding| binding.sign_in)
                    .eq([BoundSignIn::Unavailable])
            }
        ));
    }

    #[test]
    fn rejects_an_own_client_whose_admitted_client_redirects_to_two_routes() {
        assert_eq!(
            oauth_client_rejection(&own_provider_granting(
                "redirect_routes = [routes::Callback, routes::Landing]"
            ))
            .to_string(),
            "the oauth client 'crate::PortalClient' acts as the admitted client 'portal_app', which redirects to more than one route, so its sign-in cannot tell which route to return to"
        );
    }

    #[test]
    fn rejects_own_keys_no_oauth_client_signs_with() {
        assert_eq!(
            oauth_client_rejection(&own_provider_admitting("Own")).to_string(),
            "the admitted client 'portal_app' declared by 'crate::Portal' verifies its assertions with ClientKeys::Own, but no #[acts_as_oauth_client(admitted_as = portal_app)] signs them"
        );
    }

    #[test]
    fn rejects_an_oauth_client_of_the_own_provider_without_token_issuance() {
        assert!(matches!(
            oauth_client_rejection(&own_client("portal", "portal_app", "PortalClient")),
            TagError::UnknownTag { ref tag, kind: TagExpectation::AdmittedClient, .. } if tag == "portal_app"
        ));
    }

    #[test]
    fn rejects_an_oauth_client_of_an_undeclared_admitted_client() {
        assert!(matches!(
            oauth_client_rejection(&format!(
                "{}{}",
                own_provider_admitting("Own"),
                own_client("portal", "kiosk_app", "PortalClient")
            )),
            TagError::UnknownTag { ref tag, kind: TagExpectation::AdmittedClient, .. } if tag == "kiosk_app"
        ));
    }

    #[test]
    fn rejects_an_oauth_client_of_an_admitted_client_with_published_keys() {
        assert_eq!(
            oauth_client_rejection(&format!(
                "{}{}",
                own_provider_admitting(
                    "Published(jwks_uri = \"https://portal.example/jwks.json\", signing = margaret::framework::jose_parameters::jws_algorithm::JwsAlgorithm::Es256)"
                ),
                own_client("portal", "portal_app", "PortalClient")
            ))
            .to_string(),
            "the oauth client 'crate::PortalClient' acts as the admitted client 'portal_app', which does not verify its assertions with ClientKeys::Own"
        );
    }

    #[test]
    fn rejects_two_oauth_clients_acting_as_one_admitted_client() {
        assert_eq!(
            oauth_client_rejection(&format!(
                "{}{}{}",
                own_provider_admitting("Own"),
                own_client("first", "portal_app", "FirstClient"),
                own_client("second", "portal_app", "SecondClient")
            ))
            .to_string(),
            "the oauth clients 'crate::FirstClient' and 'crate::SecondClient' both act as the admitted client 'portal_app', so the provider cannot tell them apart"
        );
    }

    #[test]
    fn resolves_an_oauth_client_tag() {
        let indexed = IndexedSource::new(&oauth_client_source());
        let pool = pool(&indexed);

        assert_eq!(
            pool.resolve(
                &tag("partner_client"),
                TagExpectation::OAuthClient,
                "the site"
            )
            .expect("the tag resolves")
            .canonical_path()
            .to_string(),
            "crate::PartnerClient"
        );
    }

    #[test]
    fn reports_a_trusted_issuer_resolved_as_an_oauth_client() {
        let indexed = IndexedSource::new(&oauth_client_source());
        let pool = pool(&indexed);

        assert!(matches!(
            pool.resolve(&tag("partner"), TagExpectation::OAuthClient, "the site"),
            Err(TagError::WrongKind { expected, found, .. })
                if expected == TagExpectation::OAuthClient
                    && found == TagKind::TrustedIssuer(TrustedIssuerKind::OidcIssuer)
        ));
    }

    #[test]
    fn rejects_an_oauth_client_of_an_issuer_without_discovery() {
        assert_eq!(
            oauth_client_rejection(&format!(
                "{JWKS_TRUST}\n{}",
                oauth_client("partner_client", "partner", "jwks", "PartnerClient")
            ))
            .to_string(),
            "the oauth client 'crate::PartnerClient' names the issuer 'jwks', which publishes no discovery document"
        );
    }

    #[test]
    fn rejects_an_oauth_client_of_an_undeclared_issuer() {
        assert!(matches!(
            oauth_client_rejection(&oauth_client(
                "partner_client",
                "partner",
                "partner",
                "PartnerClient"
            )),
            TagError::UnknownTag { ref tag, kind: TagExpectation::TrustedIssuer, .. } if tag == "partner"
        ));
    }

    #[test]
    fn rejects_an_oauth_client_whose_issuer_is_another_oauth_client() {
        assert_eq!(
            oauth_client_rejection(&format!(
                "{}{}",
                oauth_client("first_client", "first", "second_client", "FirstClient"),
                oauth_client("second_client", "second", "first_client", "SecondClient")
            ))
            .to_string(),
            "the oauth client 'crate::FirstClient' references the tag 'second_client', which is a oauth client, not a trusted issuer"
        );
    }

    #[test]
    fn rejects_two_clients_of_one_issuer_with_one_client_identifier() {
        assert_eq!(
            oauth_client_rejection(&format!(
                "{PARTNER_TRUST}#[verifies_tokens_from_issuer(partner_api, audience = \"other\", issuer = \"https://partner.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Discovered)]\nstruct ApiIssuer;\n{}{}",
                oauth_client("first_client", "shared", "partner", "FirstClient"),
                oauth_client("second_client", "shared", "partner_api", "SecondClient")
            ))
            .to_string(),
            "the oauth clients 'crate::FirstClient' and 'crate::SecondClient' both identify as 'shared' at the issuer 'https://partner.example', so the issuer cannot tell them apart"
        );
    }

    #[test]
    fn binds_one_client_identifier_at_two_issuers() {
        assert!(with_bindings(
            &format!(
                "{PARTNER_TRUST}#[verifies_tokens_from_issuer(other, audience = \"api\", issuer = \"https://other.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Discovered)]\nstruct OtherIssuer;\n{}{}",
                oauth_client("first_client", "shared", "partner", "FirstClient"),
                oauth_client("second_client", "shared", "other", "SecondClient")
            ),
            |bindings| matches!(bindings.as_deref(), Ok([_, _]))
        ));
    }

    #[test]
    fn rejects_a_tag_declared_by_a_trust_and_an_oauth_client() {
        assert_eq!(
            error_for(&format!(
                "{PARTNER_TRUST}{}",
                oauth_client("partner", "partner", "partner", "PartnerClient")
            )),
            "the tag 'partner' is declared more than once: by 'crate::Issuer' and by 'crate::PartnerClient'"
        );
    }

    #[test]
    fn resolves_a_middleware_tag_to_its_handler() {
        let indexed = IndexedSource::new(
            "#[handles_middleware_attribute(attribute = logged)]\nstruct RequestLog;\n",
        );
        let pool = pool(&indexed);

        assert_eq!(
            pool.resolve(&tag("logged"), TagExpectation::Middleware, "the site")
                .expect("the tag resolves")
                .canonical_path()
                .to_string(),
            "crate::RequestLog"
        );
    }

    #[test]
    fn lists_middleware_handlers_with_their_tags_in_a_stable_order() {
        let indexed = IndexedSource::new(&format!(
            "#[handles_middleware_attribute(attribute = traced)]\nstruct Tracer;\n\n#[handles_middleware_attribute(attribute = logged)]\nstruct RequestLog;\n\n{JWKS_TRUST}"
        ));
        let pool = pool(&indexed);
        let handlers: Vec<String> = pool
            .middleware_handlers()
            .iter()
            .map(|handler| format!("{}={}", handler.tag, handler.item.canonical_path()))
            .collect();

        assert_eq!(
            handlers,
            vec![
                "logged=crate::RequestLog".to_string(),
                "traced=crate::Tracer".to_string(),
            ]
        );
    }

    #[test]
    fn resolves_a_jwks_endpoint_tag_as_a_trusted_issuer() {
        let indexed = IndexedSource::new(JWKS_TRUST);
        let pool = pool(&indexed);

        assert_eq!(
            pool.resolve(&tag("jwks"), TagExpectation::TrustedIssuer, "the site")
                .expect("the tag resolves")
                .canonical_path()
                .to_string(),
            "crate::JwksEndpoint"
        );
    }

    #[test]
    fn resolves_an_oidc_issuer_tag_as_a_trusted_issuer() {
        let indexed = IndexedSource::new(PARTNER_TRUST);
        let pool = pool(&indexed);

        assert_eq!(
            pool.resolve(&tag("partner"), TagExpectation::TrustedIssuer, "the site")
                .expect("the tag resolves")
                .canonical_path()
                .to_string(),
            "crate::Issuer"
        );
    }

    #[test]
    fn reports_an_unknown_tag() {
        let indexed = IndexedSource::new(JWKS_TRUST);
        let pool = pool(&indexed);

        assert!(
            pool.resolve(&tag("missing"), TagExpectation::TrustedIssuer, "the site")
                .err()
                .expect("the tag is unknown")
                .to_string()
                .contains("no trusted issuer declares")
        );
    }

    #[test]
    fn reports_a_tag_of_the_wrong_kind() {
        let indexed = IndexedSource::new(JWKS_TRUST);
        let pool = pool(&indexed);

        assert!(
            pool.resolve(&tag("jwks"), TagExpectation::Middleware, "the site")
                .err()
                .expect("the tag is the wrong kind")
                .to_string()
                .contains("is a trusted issuer with published keys, not a middleware handler")
        );
    }

    #[test]
    fn reports_an_oidc_issuer_tag_of_the_wrong_kind() {
        let indexed = IndexedSource::new(PARTNER_TRUST);
        let pool = pool(&indexed);

        assert!(
            pool.resolve(&tag("partner"), TagExpectation::Middleware, "the site")
                .err()
                .expect("the tag is the wrong kind")
                .to_string()
                .contains("is a trusted issuer with discovered keys, not a middleware handler")
        );
    }

    #[test]
    fn rejects_a_tag_declared_by_both_a_jwks_endpoint_and_a_middleware() {
        assert_eq!(
            error_for(&format!(
                "{JWKS_TRUST}\n#[handles_middleware_attribute(attribute = jwks)]\nstruct Handler;\n"
            )),
            "the tag 'jwks' is declared more than once: by 'crate::JwksEndpoint' and by 'crate::Handler'"
        );
    }

    #[test]
    fn rejects_a_tag_declared_by_two_trusts() {
        assert_eq!(
            error_for(
                "#[verifies_tokens_from_issuer(shared, audience = \"a\", issuer = \"https://a.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Discovered)]\nstruct First;\n#[verifies_tokens_from_issuer(shared, audience = \"b\", issuer = \"https://b.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Discovered)]\nstruct Second;\n"
            ),
            "the tag 'shared' is declared more than once: by 'crate::First' and by 'crate::Second'"
        );
    }

    #[test]
    fn rejects_a_middleware_handler_without_a_tag() {
        assert!(
            error_for("#[handles_middleware_attribute]\nstruct RequestLog;\n")
                .contains("does not name a tag")
        );
    }

    #[test]
    fn rejects_a_middleware_tag_that_is_not_a_plain_name() {
        assert!(
            error_for("#[handles_middleware_attribute(attribute = tags::guard)]\nstruct Guard;\n")
                .contains("not a single plain name")
        );
    }

    #[test]
    fn reports_a_non_path_middleware_tag_argument() {
        let error = rejection_for(
            "#[handles_middleware_attribute(attribute = \"logged\")]\nstruct RequestLog;\n",
        );

        assert!(matches!(
            error,
            TagError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument {
                    ref key,
                    ref expected,
                    ..
                }
            } if key == "attribute" && expected == "path"
        ));
    }

    #[test]
    fn reports_unparseable_attribute_arguments() {
        assert!(
            error_for("#[handles_middleware_attribute(= 5)]\nstruct RequestLog;\n")
                .contains("failed to index")
        );
    }

    const EXCHANGING_PROVIDER: &str = "#[issues_tokens(provider, audience = \"session\", issuer = \"https://issuer.example\")]\nstruct ExchangeIssuer;\n#[issues_resource_tokens(artifacts, audience = \"artifacts\")]\nstruct ExchangedArtifacts;\n#[admits_oauth_client(deployer_app, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::None, client_id = \"deployer\", resources = [artifacts], token_exchange(scopes = [\"deploy\"]))]\nstruct Deployer;\n";

    fn bindings_of_exchangers(lib_source: &str) -> Result<Vec<String>, TagError> {
        let indexed = IndexedSource::new(lib_source);
        let issuance =
            DeclaredTokenIssuance::read(&indexed.index).expect("the token issuance is read");
        let resources = DeclaredResourceIssuances::read(&indexed.index, &issuance)
            .expect("the resources are read");
        let admitted = DeclaredAcceptedClients::read(&indexed.index, &issuance, &resources)
            .expect("the admitted clients are read");
        let trusts = trusts(&indexed);
        let pool = pool(&indexed);

        pool.subject_token_exchanger_bindings(&indexed.index, &trusts, &admitted)
            .map(|bindings| {
                bindings
                    .iter()
                    .map(|binding| format!("{}={}", binding.issuer.trust.tag, binding.declaring))
                    .collect()
            })
    }

    fn exchanger_bindings(lib_source: &str) -> Result<Vec<String>, TagError> {
        bindings_of_exchangers(&format!("{EXCHANGING_PROVIDER}{lib_source}"))
    }

    #[test]
    fn rejects_a_subject_token_exchanger_no_admitted_client_exchanges_with() {
        assert_eq!(
            bindings_of_exchangers(&format!(
                "{JWKS_TRUST}\n#[exchanges_tokens_from(issuer = jwks)]\nstruct CiExchanger;\n"
            ))
            .expect_err("nothing exchanges a subject token")
            .to_string(),
            "the subject token exchanger 'crate::CiExchanger' is never used: no admitted client may exchange tokens"
        );
    }

    #[test]
    fn rejects_a_token_exchange_without_a_subject_token_exchanger() {
        assert_eq!(
            exchanger_bindings("")
                .expect_err("no exchanger accepts a subject token")
                .to_string(),
            "the admitted client 'crate::Deployer' may exchange tokens, but no #[exchanges_tokens_from] exchanger accepts any subject token"
        );
    }

    #[test]
    fn binds_a_subject_token_exchanger_to_its_issuer() {
        assert_eq!(
            exchanger_bindings(&format!(
                "{JWKS_TRUST}\n#[exchanges_tokens_from(issuer = jwks)]\nstruct CiExchanger;\n"
            ))
            .expect("the exchanger names a trusted issuer"),
            vec!["jwks=crate::CiExchanger".to_string()]
        );
    }

    #[test]
    fn rejects_a_subject_token_exchanger_without_an_issuer() {
        assert!(matches!(
            exchanger_bindings("#[exchanges_tokens_from]\nstruct CiExchanger;\n"),
            Err(TagError::MalformedSubjectTokenExchangerIssuer { ref concrete })
                if concrete == "crate::CiExchanger"
        ));
    }

    #[test]
    fn reports_a_subject_token_exchanger_issuer_that_is_not_a_path() {
        assert!(matches!(
            exchanger_bindings("#[exchanges_tokens_from(issuer = \"ci\")]\nstruct CiExchanger;\n"),
            Err(TagError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument { ref key, .. }
            }) if key == "issuer"
        ));
    }

    #[test]
    fn reports_unparseable_subject_token_exchanger_arguments() {
        assert!(matches!(
            exchanger_bindings("#[exchanges_tokens_from(= 5)]\nstruct CiExchanger;\n"),
            Err(TagError::Index {
                source: AttributeError::Arguments(AttributeArgumentsError::Malformed {
                    ref attribute_path,
                    ..
                })
            }) if attribute_path == "exchanges_tokens_from"
        ));
    }

    #[test]
    fn rejects_a_subject_token_exchanger_of_an_undeclared_issuer() {
        assert!(matches!(
            exchanger_bindings("#[exchanges_tokens_from(issuer = ci)]\nstruct CiExchanger;\n"),
            Err(TagError::UnknownTag { ref tag, kind: TagExpectation::TrustedIssuer, .. })
                if tag == "ci"
        ));
    }

    #[test]
    fn rejects_a_subject_token_exchanger_of_an_oauth_client() {
        assert_eq!(
            exchanger_bindings(&format!(
                "{}\n#[exchanges_tokens_from(issuer = partner_client)]\nstruct Exchanger;\n",
                oauth_client_source()
            ))
            .expect_err("an oauth client is not a trusted issuer")
            .to_string(),
            "the subject token exchanger 'crate::Exchanger' references the tag 'partner_client', which is a oauth client, not a trusted issuer"
        );
    }

    #[test]
    fn rejects_a_second_subject_token_exchanger_of_an_issuer() {
        assert_eq!(
            exchanger_bindings(&format!(
                "{JWKS_TRUST}\n#[exchanges_tokens_from(issuer = jwks)]\nstruct First;\n\n#[exchanges_tokens_from(issuer = jwks)]\nstruct Second;\n"
            ))
            .expect_err("an issuer has one exchanger")
            .to_string(),
            "the issuer 'jwks' has more than one subject token exchanger: 'crate::First' and 'crate::Second'"
        );
    }

    fn bearer_source(declarations: &str, parameter: &str) -> String {
        format!(
            "{declarations}\n#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct RunnerProvider;\n\nimpl RunnerProvider {{\n    #[infer_from_request]\n    fn infer(&self, {parameter}) -> anyhow::Result<AuthenticatedUserOutcome<User>> {{}}\n}}\n"
        )
    }

    const OWN_TOKENS: &str = "#[issues_tokens(provider, audience = \"session\", issuer = \"https://issuer.example\")]\nstruct Issuer;\n#[issues_resource_tokens(attachments, audience = \"attachments\")]\nstruct Attachments;\n";

    #[test]
    fn verifies_bearer_tokens_for_the_own_issuance_and_resources_they_name() {
        let indexed = IndexedSource::new(&format!(
            "{}{}",
            bearer_source(
                OWN_TOKENS,
                "#[bearer_token(issuer = provider)] session: Option<Session>"
            ),
            "struct Uploader;\n\nimpl Uploader {\n    fn upload(&self, #[bearer_token(resource = attachments)] token: Option<Upload>) {}\n}\n"
        ));
        let pool = pool(&indexed);

        assert!(pool.verifies_bearer_tokens_for(&tag("provider")));
        assert!(pool.verifies_bearer_tokens_for(&tag("attachments")));
        assert!(!pool.verifies_bearer_tokens_for(&tag("partner")));
    }

    #[test]
    fn finds_the_addressee_scanned_for_the_attributes_of_a_parameter() {
        let indexed = IndexedSource::new(&bearer_source(
            PARTNER_TRUST,
            "#[bearer_token(issuer = partner)] token: Option<Token>",
        ));
        let pool = pool(&indexed);
        let parameter_attributes = indexed
            .item("RunnerProvider")
            .methods()
            .iter()
            .flat_map(IndexedMethod::parameters)
            .map(IndexedParameter::attributes)
            .find(|attributes| !attributes.is_empty())
            .expect("the bearer token parameter is indexed");

        assert!(matches!(
            pool.bearer_token(parameter_attributes),
            Some(BearerTokenAddressee::Issuer(issuer)) if issuer.to_string() == "partner"
        ));
        assert!(pool.bearer_token(&[]).is_none());
    }

    #[test]
    fn rejects_a_bearer_token_introspected_through_an_own_client() {
        assert_eq!(
            error_for(&bearer_source(
                &format!(
                    "{}{}",
                    own_provider_admitting("Own"),
                    own_client("portal", "portal_app", "PortalClient")
                ),
                "#[bearer_token(client = portal)] token: Option<Token>",
            )),
            "argument #1 of 'crate::RunnerProvider::infer' introspects its bearer token through the oauth client 'portal', which acts as a client of this application's own provider; verify the provider's resource tokens with #[bearer_token(resource = <tag>)] instead"
        );
    }

    #[test]
    fn rejects_a_bearer_token_without_an_addressee() {
        assert!(error_for(&bearer_source(PARTNER_TRUST, "#[bearer_token] token: Option<Token>")).ends_with(
            "argument #1 of 'crate::RunnerProvider::infer' must be exactly one of `client = <tag>`, `issuer = <tag>` or `resource = <tag>`"
        ));
    }

    #[test]
    fn reports_bearer_token_arguments_that_do_not_parse() {
        assert!(
            error_for(&bearer_source(
                PARTNER_TRUST,
                "#[bearer_token(= 5)] token: Option<Token>"
            ))
            .contains("could not be parsed")
        );
    }

    #[test]
    fn rejects_a_bearer_token_of_an_undeclared_issuer() {
        assert!(error_for(&bearer_source(PARTNER_TRUST, "#[bearer_token(issuer = undeclared)] token: Option<Token>")).ends_with(
            "references the tag 'undeclared', which no token issuance or trusted issuer declares"
        ));
    }

    #[test]
    fn rejects_a_bearer_token_of_a_middleware_tag() {
        assert!(error_for(&bearer_source(
            "#[singleton]\n#[handles_middleware_attribute(attribute = logged)]\nstruct RequestLog;\n",
            "#[bearer_token(issuer = logged)] token: Option<Token>",
        ))
        .ends_with(
            "references the tag 'logged', which is a middleware handler, not a token issuance or trusted issuer"
        ));
    }

    #[test]
    fn rejects_an_introspected_bearer_token_of_an_unknown_client() {
        assert!(
            error_for(&bearer_source(
                PARTNER_TRUST,
                "#[bearer_token(client = missing_client)] token: Option<Token>"
            ))
            .ends_with("references the tag 'missing_client', which no oauth client declares")
        );
    }

    #[test]
    fn rejects_a_bearer_token_of_a_resource_that_names_the_token_issuance() {
        assert!(
            error_for(&bearer_source(
                OWN_TOKENS,
                "#[bearer_token(resource = provider)] token: Option<Token>"
            ))
            .ends_with("references the tag 'provider', which is a token issuance, not a resource")
        );
    }

    #[test]
    fn rejects_an_introspected_bearer_token_of_an_admitted_client() {
        assert!(
            error_for(&bearer_source(
                &own_provider_admitting("Own"),
                "#[bearer_token(client = portal_app)] token: Option<Token>"
            ))
            .ends_with(
                "references the tag 'portal_app', which is a client admitted by the provider, not a oauth client"
            )
        );
    }

    #[test]
    fn reports_a_resource_tag_where_an_admitted_client_is_expected() {
        let indexed = IndexedSource::new(&own_provider_admitting("Own"));

        assert_eq!(
            pool(&indexed)
                .resolve(
                    &tag("artifacts"),
                    TagExpectation::AdmittedClient,
                    "the site"
                )
                .err()
                .expect("a resource is not an admitted client")
                .to_string(),
            "the site references the tag 'artifacts', which is a resource, not a client admitted by the provider"
        );
    }

    #[test]
    fn reports_a_token_issuance_tag_where_a_trusted_issuer_is_expected() {
        let indexed = IndexedSource::new(
            "#[issues_tokens(provider, audience = \"session\", issuer = \"https://issuer.example\")]\nstruct Issuer;\n",
        );

        assert_eq!(
            pool(&indexed)
                .resolve(&tag("provider"), TagExpectation::TrustedIssuer, "the site")
                .err()
                .expect("the token issuance is not a trusted issuer")
                .to_string(),
            "the site references the tag 'provider', which is a token issuance, not a trusted issuer"
        );
    }

    #[test]
    fn rejects_a_token_issuance_that_shares_the_tag_of_a_trusted_issuer() {
        assert_eq!(
            error_for(&format!(
                "{PARTNER_TRUST}#[issues_tokens(partner, audience = \"session\", issuer = \"https://issuer.example\")]\nstruct OwnIssuer;\n"
            )),
            "the tag 'partner' is declared more than once: by 'crate::Issuer' and by 'crate::OwnIssuer'"
        );
    }

    #[test]
    fn rejects_a_resource_that_shares_the_tag_of_the_token_issuance() {
        assert_eq!(
            error_for(
                "#[issues_tokens(provider, audience = \"session\", issuer = \"https://issuer.example\")]\nstruct Issuer;\n#[issues_resource_tokens(provider, audience = \"artifacts\")]\nstruct Artifacts;\n"
            ),
            "the tag 'provider' is declared more than once: by 'crate::Issuer' and by 'crate::Artifacts'"
        );
    }

    #[test]
    fn rejects_an_admitted_client_that_shares_the_tag_of_a_resource() {
        assert_eq!(
            error_for(
                "#[issues_tokens(provider, audience = \"session\", issuer = \"https://issuer.example\")]\nstruct Issuer;\n#[issues_resource_tokens(artifacts, audience = \"artifacts\")]\nstruct Artifacts;\n#[admits_oauth_client(artifacts, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::None, client_id = \"portal\", resources = [artifacts])]\nstruct Portal;\n"
            ),
            "the tag 'artifacts' is declared more than once: by 'crate::Artifacts' and by 'crate::Portal'"
        );
    }

    #[test]
    fn rejects_a_bearer_token_of_an_undeclared_resource() {
        assert!(
            error_for(&bearer_source(
                OWN_TOKENS,
                "#[bearer_token(resource = reports)] token: Option<Token>"
            ))
            .ends_with("references the tag 'reports', which no resource declares")
        );
    }
}
