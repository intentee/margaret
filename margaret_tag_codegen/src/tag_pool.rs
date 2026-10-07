use std::collections::HashMap;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::tag::Tag;
use margaret_oauth_client_codegen::declared_oauth_clients::DeclaredOAuthClients;
use margaret_trusted_issuer_codegen::declared_trusts::DeclaredTrusts;
use margaret_trusted_issuer_codegen::trusted_issuer_binding::TrustedIssuerBinding;
use margaret_trusted_issuer_codegen::trusted_issuer_group::TrustedIssuerGroup;

use crate::oauth_client_binding::OAuthClientBinding;
use crate::read_middleware_attribute::read_middleware_attribute;
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

pub struct TagPool<'index> {
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

        Ok(Self { entries })
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
    /// issuer, `TagError::OAuthClientIssuerNotDiscovered` for an issuer without discovery, and
    /// `TagError::DuplicateOAuthClient` for two clients of one issuer with one client identifier.
    pub fn oauth_client_bindings<'declarations>(
        &self,
        trusts: &'declarations DeclaredTrusts<'index>,
        clients: &'declarations DeclaredOAuthClients<'index>,
    ) -> Result<Vec<OAuthClientBinding<'declarations, 'index>>, TagError> {
        let mut bindings: Vec<OAuthClientBinding<'declarations, 'index>> = Vec::new();

        for client in &clients.clients {
            let site = format!("the oauth client '{}'", client.anchor.canonical_path());
            let issuer = match trusts.binding(&client.issuer) {
                Some(binding) if matches!(binding.group, TrustedIssuerGroup::Discovered { .. }) => {
                    binding
                }
                Some(binding) => {
                    return Err(TagError::OAuthClientIssuerNotDiscovered {
                        issuer: binding.trust.tag.to_string(),
                        site,
                    });
                }
                None => return Err(self.unresolved_trusted_issuer(&client.issuer, site)),
            };

            if let Some(existing) = bindings.iter().find(|existing| {
                existing.client.client_id == client.client_id
                    && existing.issuer.group.issuer().as_str() == issuer.group.issuer().as_str()
            }) {
                return Err(TagError::DuplicateOAuthClient {
                    client_id: client.client_id.to_string(),
                    first: existing.client.anchor.canonical_path().to_string(),
                    issuer: issuer.group.issuer().to_string(),
                    second: client.anchor.canonical_path().to_string(),
                });
            }

            bindings.push(OAuthClientBinding { client, issuer });
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
        match self.entries.get(tag) {
            Some(entry) if expected.admits(entry.kind) => Ok(entry.item),
            Some(entry) => Err(wrong_kind(tag, expected, site.to_string(), entry)),
            None => Err(unknown_tag(tag, expected, site.to_string())),
        }
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
    ) -> Result<Vec<SubjectTokenExchangerBinding<'trusts, 'index>>, TagError> {
        let mut bindings: Vec<SubjectTokenExchangerBinding<'trusts, 'index>> = Vec::new();

        for matched in index.select_framework_attribute(FrameworkAttribute::ExchangesSubjectTokens)
        {
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
                return Err(self.unresolved_trusted_issuer(&issuer, site));
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

        Ok(bindings)
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

    fn unresolved_trusted_issuer(&self, issuer: &Tag, site: String) -> TagError {
        match self.entries.get(issuer) {
            Some(entry) => wrong_kind(issuer, TagExpectation::TrustedIssuer, site, entry),
            None => unknown_tag(issuer, TagExpectation::TrustedIssuer, site),
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::Path;
    use syn::parse_str;

    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_attributes::attribute_error::AttributeError;
    use margaret_attributes::tag::Tag;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_oauth_client_codegen::declared_oauth_clients::DeclaredOAuthClients;
    use margaret_trusted_issuer_codegen::declared_trusts::DeclaredTrusts;

    use crate::tag_error::TagError;
    use crate::tag_expectation::TagExpectation;
    use crate::tag_kind::TagKind;
    use crate::tag_pool::TagPool;
    use crate::trusted_issuer_kind::TrustedIssuerKind;

    const PARTNER_TRUST: &str = "#[trusts_oidc_issuer(partner, audience = \"api\", issuer = \"https://partner.example\")]\nstruct Issuer;\n";

    const JWKS_TRUST: &str = "#[provides_jwks_endpoint(jwks, audience = \"api\", issuer = \"https://jwks.example\", jwks_uri = \"https://jwks.example/keys\")]\nstruct JwksEndpoint;\n";

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

    fn pool(indexed: &IndexedSource) -> TagPool<'_> {
        TagPool::collect(&indexed.index, &trusts(indexed), &clients(indexed))
            .expect("the pool collects")
    }

    fn rejection_for(lib_source: &str) -> TagError {
        let indexed = IndexedSource::new(lib_source);

        TagPool::collect(&indexed.index, &trusts(&indexed), &clients(&indexed))
            .err()
            .expect("the pool fails to collect")
    }

    fn error_for(lib_source: &str) -> String {
        rejection_for(lib_source).to_string()
    }

    fn oauth_client(tag: &str, client_id: &str, issuer: &str, anchor: &str) -> String {
        format!(
            "#[oauth_client({tag}, authentication = private_key_jwt, client_id = \"{client_id}\", issuer = {issuer})]\nstruct {anchor};\n"
        )
    }

    fn oauth_client_source() -> String {
        format!(
            "{PARTNER_TRUST}\n{}",
            oauth_client("partner_client", "partner", "partner", "PartnerClient")
        )
    }

    fn oauth_client_rejection(lib_source: &str) -> TagError {
        let indexed = IndexedSource::new(lib_source);
        let trusts = trusts(&indexed);
        let clients = clients(&indexed);
        let pool = TagPool::collect(&indexed.index, &trusts, &clients).expect("the pool collects");

        pool.oauth_client_bindings(&trusts, &clients)
            .err()
            .expect("the oauth client is rejected")
    }

    #[test]
    fn binds_an_oauth_client_to_its_discovered_issuer() {
        let indexed = IndexedSource::new(&oauth_client_source());
        let trusts = trusts(&indexed);
        let clients = clients(&indexed);
        let pool = TagPool::collect(&indexed.index, &trusts, &clients).expect("the pool collects");
        let bindings = pool
            .oauth_client_bindings(&trusts, &clients)
            .expect("the oauth client names a discovered issuer");

        assert_eq!(bindings.len(), 1);
        assert_eq!(
            bindings[0].client.anchor.canonical_path().to_string(),
            "crate::PartnerClient"
        );
        assert_eq!(bindings[0].issuer.trust.tag.to_string(), "partner");
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
                "{PARTNER_TRUST}#[trusts_oidc_issuer(partner_api, audience = \"other\", issuer = \"https://partner.example\")]\nstruct ApiIssuer;\n{}{}",
                oauth_client("first_client", "shared", "partner", "FirstClient"),
                oauth_client("second_client", "shared", "partner_api", "SecondClient")
            ))
            .to_string(),
            "the oauth clients 'crate::FirstClient' and 'crate::SecondClient' both identify as 'shared' at the issuer 'https://partner.example', so the issuer cannot tell them apart"
        );
    }

    #[test]
    fn binds_one_client_identifier_at_two_issuers() {
        let indexed = IndexedSource::new(&format!(
            "{PARTNER_TRUST}#[trusts_oidc_issuer(other, audience = \"api\", issuer = \"https://other.example\")]\nstruct OtherIssuer;\n{}{}",
            oauth_client("first_client", "shared", "partner", "FirstClient"),
            oauth_client("second_client", "shared", "other", "SecondClient")
        ));
        let trusts = trusts(&indexed);
        let clients = clients(&indexed);
        let pool = TagPool::collect(&indexed.index, &trusts, &clients).expect("the pool collects");

        assert_eq!(
            pool.oauth_client_bindings(&trusts, &clients)
                .expect("distinct issuers tell the clients apart")
                .len(),
            2
        );
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
                .contains("is a jwks endpoint provider, not a middleware handler")
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
                .contains("oidc issuer, not a middleware handler")
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
                "#[trusts_oidc_issuer(shared, audience = \"a\", issuer = \"https://a.example\")]\nstruct First;\n#[trusts_oidc_issuer(shared, audience = \"b\", issuer = \"https://b.example\")]\nstruct Second;\n"
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

    fn exchanger_bindings(lib_source: &str) -> Result<Vec<String>, TagError> {
        let indexed = IndexedSource::new(lib_source);
        let trusts = trusts(&indexed);
        let pool = TagPool::collect(&indexed.index, &trusts, &clients(&indexed))
            .expect("the pool collects");

        pool.subject_token_exchanger_bindings(&indexed.index, &trusts)
            .map(|bindings| {
                bindings
                    .iter()
                    .map(|binding| format!("{}={}", binding.issuer.trust.tag, binding.declaring))
                    .collect()
            })
    }

    #[test]
    fn binds_a_subject_token_exchanger_to_its_issuer() {
        assert_eq!(
            exchanger_bindings(&format!(
                "{JWKS_TRUST}\n#[exchanges_subject_tokens(issuer = jwks)]\nstruct CiExchanger;\n"
            ))
            .expect("the exchanger names a trusted issuer"),
            vec!["jwks=crate::CiExchanger".to_string()]
        );
    }

    #[test]
    fn rejects_a_subject_token_exchanger_without_an_issuer() {
        assert!(matches!(
            exchanger_bindings("#[exchanges_subject_tokens]\nstruct CiExchanger;\n"),
            Err(TagError::MalformedSubjectTokenExchangerIssuer { ref concrete })
                if concrete == "crate::CiExchanger"
        ));
    }

    #[test]
    fn reports_a_subject_token_exchanger_issuer_that_is_not_a_path() {
        assert!(matches!(
            exchanger_bindings("#[exchanges_subject_tokens(issuer = \"ci\")]\nstruct CiExchanger;\n"),
            Err(TagError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument { ref key, .. }
            }) if key == "issuer"
        ));
    }

    #[test]
    fn reports_unparseable_subject_token_exchanger_arguments() {
        assert!(matches!(
            exchanger_bindings("#[exchanges_subject_tokens(= 5)]\nstruct CiExchanger;\n"),
            Err(TagError::Index {
                source: AttributeError::Arguments(AttributeArgumentsError::Malformed {
                    ref attribute_path,
                    ..
                })
            }) if attribute_path == "exchanges_subject_tokens"
        ));
    }

    #[test]
    fn rejects_a_subject_token_exchanger_of_an_undeclared_issuer() {
        assert!(matches!(
            exchanger_bindings("#[exchanges_subject_tokens(issuer = ci)]\nstruct CiExchanger;\n"),
            Err(TagError::UnknownTag { ref tag, kind: TagExpectation::TrustedIssuer, .. })
                if tag == "ci"
        ));
    }

    #[test]
    fn rejects_a_subject_token_exchanger_of_an_oauth_client() {
        assert_eq!(
            exchanger_bindings(&format!(
                "{}\n#[exchanges_subject_tokens(issuer = partner_client)]\nstruct Exchanger;\n",
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
                "{JWKS_TRUST}\n#[exchanges_subject_tokens(issuer = jwks)]\nstruct First;\n\n#[exchanges_subject_tokens(issuer = jwks)]\nstruct Second;\n"
            ))
            .expect_err("an issuer has one exchanger")
            .to_string(),
            "the issuer 'jwks' has more than one subject token exchanger: 'crate::First' and 'crate::Second'"
        );
    }
}
