use std::collections::HashMap;

use syn::Path;

use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::field_base::field_base;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::name_allocator::NameAllocator;
use margaret_attributes::tag::Tag;

use crate::oauth_client_arguments::OAuthClientArguments;
use crate::oauth_client_binding::OAuthClientBinding;
use crate::read_middleware_attribute::read_middleware_attribute;
use crate::subject_token_exchanger_binding::SubjectTokenExchangerBinding;
use crate::tag_declaration::TagDeclaration;
use crate::tag_declaring_attribute::TagDeclaringAttribute;
use crate::tag_error::TagError;
use crate::tag_expectation::TagExpectation;
use crate::tagged_item::TaggedItem;
use crate::trusted_issuer_binding::TrustedIssuerBinding;
use crate::trusted_issuer_kind::TrustedIssuerKind;

struct DeclaredTag {
    declaration: TagDeclaration,
    path: Option<Path>,
}

struct TagEntry<'index> {
    declaration: TagDeclaration,
    item: &'index IndexedItem,
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
        found: entry.declaration.kind(),
    }
}

fn positional_tag(
    args: &AttributeArgs,
    declaration: TagDeclaration,
) -> Result<DeclaredTag, TagError> {
    Ok(DeclaredTag {
        declaration,
        path: args.interpret(|reader| Ok::<_, TagError>(reader.take_positional_path()))?,
    })
}

fn declared_tag(
    args: &AttributeArgs,
    attribute: TagDeclaringAttribute,
    concrete: &str,
) -> Result<DeclaredTag, TagError> {
    match attribute {
        TagDeclaringAttribute::HandlesMiddlewareAttribute => Ok(DeclaredTag {
            declaration: TagDeclaration::Middleware,
            path: read_middleware_attribute(args)?,
        }),
        TagDeclaringAttribute::OAuthClient => {
            let OAuthClientArguments { issuer, tag } = OAuthClientArguments::read(args)?;
            let issuer = issuer.as_ref().and_then(Tag::from_path).ok_or_else(|| {
                TagError::MalformedOAuthClientIssuer {
                    concrete: concrete.to_string(),
                }
            })?;

            Ok(DeclaredTag {
                declaration: TagDeclaration::OAuthClient { issuer },
                path: tag,
            })
        }
        TagDeclaringAttribute::ProvidesJwksEndpoint => positional_tag(
            args,
            TagDeclaration::TrustedIssuer(TrustedIssuerKind::JwksEndpoint),
        ),
        TagDeclaringAttribute::TrustsOidcIssuer => positional_tag(
            args,
            TagDeclaration::TrustedIssuer(TrustedIssuerKind::OidcIssuer),
        ),
    }
}

fn collect_declarations<'index>(
    index: &'index AttributeIndex,
    attribute: TagDeclaringAttribute,
    entries: &mut HashMap<Tag, TagEntry<'index>>,
) -> Result<(), TagError> {
    for matched in index.select_framework_attribute(attribute.framework_attribute()) {
        let item = matched.item();
        let concrete = item.canonical_path().to_string();
        let DeclaredTag { declaration, path } =
            declared_tag(matched.args()?, attribute, &concrete)?;
        let kind = declaration.kind();
        let path = path.ok_or_else(|| TagError::MissingTag {
            concrete: concrete.clone(),
            kind,
        })?;
        let tag = Tag::from_path(&path).ok_or_else(|| TagError::MalformedTag {
            concrete: concrete.clone(),
            kind,
        })?;

        if let Some(existing) = entries.get(&tag) {
            return Err(TagError::DuplicateTag {
                tag: tag.to_string(),
                first: existing.item.canonical_path().to_string(),
                second: concrete,
            });
        }

        entries.insert(tag, TagEntry { declaration, item });
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
    pub fn collect(index: &'index AttributeIndex) -> Result<Self, TagError> {
        let mut entries: HashMap<Tag, TagEntry<'index>> = HashMap::new();

        for attribute in TagDeclaringAttribute::ALL {
            collect_declarations(index, attribute, &mut entries)?;
        }

        Ok(Self { entries })
    }

    #[must_use]
    pub fn middleware_handlers(&self) -> Vec<TaggedItem<'index>> {
        let mut handlers: Vec<TaggedItem<'index>> = self
            .sorted_entries()
            .into_iter()
            .filter(|sorted| TagExpectation::Middleware.admits(sorted.entry.declaration.kind()))
            .map(|SortedEntry { entry, tag }| TaggedItem {
                item: entry.item,
                kind: entry.declaration.kind(),
                tag: tag.clone(),
            })
            .collect();

        handlers.sort_by(|left, right| left.item.canonical_path().cmp(right.item.canonical_path()));

        handlers
    }

    /// # Errors
    ///
    /// Returns `TagError::UnknownTag` or `TagError::WrongKind` for an issuer that is not a trusted
    /// issuer, and `TagError::OAuthClientIssuerNotDiscovered` for an issuer without discovery.
    pub fn oauth_client_bindings<'bindings>(
        &self,
        trusted_issuers: &'bindings [TrustedIssuerBinding],
    ) -> Result<Vec<OAuthClientBinding<'bindings>>, TagError> {
        let mut bindings = Vec::new();

        for SortedEntry { entry, tag } in self.sorted_entries() {
            let TagDeclaration::OAuthClient { issuer } = &entry.declaration else {
                continue;
            };
            let declaring = entry.item.canonical_path();
            let site = format!("the oauth client '{declaring}'");
            let issuer = match trusted_issuers
                .iter()
                .find(|binding| binding.tag == *issuer)
            {
                Some(binding) if binding.kind == TrustedIssuerKind::OidcIssuer => binding,
                Some(binding) => {
                    return Err(TagError::OAuthClientIssuerNotDiscovered {
                        issuer: binding.tag.to_string(),
                        site,
                    });
                }
                None => {
                    return Err(match self.entries.get(issuer) {
                        Some(entry) => {
                            wrong_kind(issuer, TagExpectation::TrustedIssuer, site, entry)
                        }
                        None => unknown_tag(issuer, TagExpectation::TrustedIssuer, site),
                    });
                }
            };

            bindings.push(OAuthClientBinding {
                declaring: declaring.clone(),
                issuer,
                tag: tag.clone(),
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
        match self.entries.get(tag) {
            Some(entry) if expected.admits(entry.declaration.kind()) => Ok(entry.item),
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
    pub fn subject_token_exchanger_bindings<'bindings>(
        &self,
        index: &AttributeIndex,
        trusted_issuers: &'bindings [TrustedIssuerBinding],
    ) -> Result<Vec<SubjectTokenExchangerBinding<'bindings>>, TagError> {
        let mut bindings: Vec<SubjectTokenExchangerBinding<'bindings>> = Vec::new();

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
            let Some(issuer) = trusted_issuers.iter().find(|binding| binding.tag == issuer) else {
                return Err(match self.entries.get(&issuer) {
                    Some(entry) => wrong_kind(&issuer, TagExpectation::TrustedIssuer, site, entry),
                    None => unknown_tag(&issuer, TagExpectation::TrustedIssuer, site),
                });
            };

            if let Some(existing) = bindings
                .iter()
                .find(|binding| binding.issuer.tag == issuer.tag)
            {
                return Err(TagError::DuplicateSubjectTokenExchanger {
                    issuer: issuer.tag.to_string(),
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

    #[must_use]
    pub fn trusted_issuer_bindings(&self) -> Vec<TrustedIssuerBinding> {
        let mut allocator = NameAllocator::new();

        self.sorted_entries()
            .into_iter()
            .filter_map(|SortedEntry { entry, tag }| match &entry.declaration {
                TagDeclaration::Middleware | TagDeclaration::OAuthClient { .. } => None,
                TagDeclaration::TrustedIssuer(kind) => Some(TrustedIssuerBinding {
                    declaring: entry.item.canonical_path().clone(),
                    kind: *kind,
                    module_segment: allocator
                        .allocate(&field_base(entry.item.canonical_path()))
                        .field()
                        .to_string(),
                    tag: tag.clone(),
                }),
            })
            .collect()
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
}

#[cfg(test)]
mod tests {
    use syn::Path;
    use syn::parse_str;

    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_attributes::attribute_error::AttributeError;
    use margaret_attributes::tag::Tag;
    use margaret_attributes_tests::indexed_source::IndexedSource;

    use crate::tag_error::TagError;
    use crate::tag_expectation::TagExpectation;
    use crate::tag_kind::TagKind;
    use crate::tag_pool::TagPool;
    use crate::trusted_issuer_kind::TrustedIssuerKind;

    fn tag(name: &str) -> Tag {
        let path: Path = parse_str(name).expect("the tag path parses");

        Tag::from_path(&path).expect("the tag is a plain name")
    }

    fn rejection_for(lib_source: &str) -> TagError {
        TagPool::collect(&IndexedSource::new(lib_source).index)
            .err()
            .expect("the pool fails to collect")
    }

    fn error_for(lib_source: &str) -> String {
        rejection_for(lib_source).to_string()
    }

    const OAUTH_CLIENT_SOURCE: &str = "#[trusts_oidc_issuer(partner)]\nstruct Issuer;\n\n#[oauth_client(partner_client, issuer = partner)]\nstruct PartnerClient;\n";

    fn oauth_client_rejection(lib_source: &str) -> TagError {
        let indexed = IndexedSource::new(lib_source);
        let pool = TagPool::collect(&indexed.index).expect("the pool collects");
        let trusted_issuers = pool.trusted_issuer_bindings();

        pool.oauth_client_bindings(&trusted_issuers)
            .err()
            .expect("the oauth client is rejected")
    }

    #[test]
    fn binds_an_oauth_client_to_its_discovered_issuer() {
        let indexed = IndexedSource::new(OAUTH_CLIENT_SOURCE);
        let pool = TagPool::collect(&indexed.index).expect("the pool collects");
        let trusted_issuers = pool.trusted_issuer_bindings();
        let bindings = pool
            .oauth_client_bindings(&trusted_issuers)
            .expect("the oauth client names a discovered issuer");

        assert_eq!(trusted_issuers.len(), 1);
        assert_eq!(bindings.len(), 1);
        assert_eq!(bindings[0].declaring.to_string(), "crate::PartnerClient");
        assert_eq!(bindings[0].issuer.module_segment, "issuer");
        assert_eq!(bindings[0].tag.to_string(), "partner_client");
    }

    #[test]
    fn resolves_an_oauth_client_tag() {
        let indexed = IndexedSource::new(OAUTH_CLIENT_SOURCE);
        let pool = TagPool::collect(&indexed.index).expect("the pool collects");

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
    fn rejects_an_oauth_client_without_a_tag() {
        assert_eq!(
            error_for("#[oauth_client(issuer = partner)]\nstruct PartnerClient;\n"),
            "the oauth client 'crate::PartnerClient' does not name a tag"
        );
    }

    #[test]
    fn rejects_an_oauth_client_without_an_issuer() {
        assert!(matches!(
            rejection_for("#[oauth_client(partner_client)]\nstruct PartnerClient;\n"),
            TagError::MalformedOAuthClientIssuer { ref concrete } if concrete == "crate::PartnerClient"
        ));
    }

    #[test]
    fn rejects_an_oauth_client_issuer_that_is_not_a_plain_name() {
        assert!(matches!(
            rejection_for(
                "#[oauth_client(partner_client, issuer = issuers::partner)]\nstruct PartnerClient;\n"
            ),
            TagError::MalformedOAuthClientIssuer { ref concrete } if concrete == "crate::PartnerClient"
        ));
    }

    #[test]
    fn reports_an_oauth_client_issuer_that_is_not_a_path() {
        assert!(matches!(
            rejection_for(
                "#[oauth_client(partner_client, issuer = \"partner\")]\nstruct PartnerClient;\n"
            ),
            TagError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument { ref key, .. }
            } if key == "issuer"
        ));
    }

    #[test]
    fn reports_an_unrecognized_argument_of_a_trusted_issuer() {
        assert!(
            error_for("#[trusts_oidc_issuer(partner, audience = ci)]\nstruct Issuer;\n")
                .contains("has an unrecognized argument 'audience'")
        );
    }

    #[test]
    fn reports_a_trusted_issuer_resolved_as_an_oauth_client() {
        let indexed = IndexedSource::new(OAUTH_CLIENT_SOURCE);
        let pool = TagPool::collect(&indexed.index).expect("the pool collects");

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
            oauth_client_rejection(
                "#[provides_jwks_endpoint(partner)]\nstruct Endpoint;\n\n#[oauth_client(partner_client, issuer = partner)]\nstruct PartnerClient;\n"
            )
            .to_string(),
            "the oauth client 'crate::PartnerClient' names the issuer 'partner', which publishes no discovery document"
        );
    }

    #[test]
    fn rejects_an_oauth_client_of_an_undeclared_issuer() {
        assert!(matches!(
            oauth_client_rejection(
                "#[oauth_client(partner_client, issuer = partner)]\nstruct PartnerClient;\n"
            ),
            TagError::UnknownTag { ref tag, kind: TagExpectation::TrustedIssuer, .. } if tag == "partner"
        ));
    }

    #[test]
    fn rejects_an_oauth_client_whose_issuer_is_another_oauth_client() {
        assert_eq!(
            oauth_client_rejection(
                "#[oauth_client(first_client, issuer = second_client)]\nstruct FirstClient;\n\n#[oauth_client(second_client, issuer = first_client)]\nstruct SecondClient;\n"
            )
            .to_string(),
            "the oauth client 'crate::FirstClient' references the tag 'second_client', which is a oauth client, not a trusted issuer"
        );
    }

    #[test]
    fn binds_a_jwks_client_tag_to_its_endpoint() {
        let indexed = IndexedSource::new("#[provides_jwks_endpoint(jwks)]\nstruct JwksEndpoint;\n");
        let pool = TagPool::collect(&indexed.index).expect("the pool collects");
        let bindings = pool.trusted_issuer_bindings();

        assert_eq!(bindings.len(), 1);
        assert_eq!(bindings[0].tag.to_string(), "jwks");
        assert_eq!(bindings[0].declaring.to_string(), "crate::JwksEndpoint");
        assert_eq!(bindings[0].kind, TrustedIssuerKind::JwksEndpoint);
        assert_eq!(bindings[0].module_segment, "jwks_endpoint");
    }

    #[test]
    fn takes_a_raw_identifier_tag_verbatim_and_derives_the_segment_from_the_endpoint() {
        let indexed =
            IndexedSource::new("#[provides_jwks_endpoint(r#async)]\nstruct AsyncEndpoint;\n");
        let pool = TagPool::collect(&indexed.index).expect("the pool collects");
        let bindings = pool.trusted_issuer_bindings();

        assert_eq!(bindings.len(), 1);
        assert_eq!(bindings[0].tag.to_string(), "r#async");
        assert_eq!(bindings[0].module_segment, "async_endpoint");
    }

    #[test]
    fn disambiguates_client_module_segments_that_collide() {
        let indexed = IndexedSource::new(
            "mod outer {\n    pub mod inner {\n        #[provides_jwks_endpoint(first)]\n        pub struct AuthEndpoint;\n    }\n}\n\nmod outer_inner {\n    #[provides_jwks_endpoint(second)]\n    pub struct AuthEndpoint;\n}\n",
        );
        let pool = TagPool::collect(&indexed.index).expect("the pool collects");

        let segments: Vec<String> = pool
            .trusted_issuer_bindings()
            .iter()
            .map(|binding| binding.module_segment.clone())
            .collect();

        assert_eq!(
            segments,
            vec![
                "outer_inner_auth_endpoint".to_string(),
                "outer_inner_auth_endpoint_2".to_string(),
            ]
        );
    }

    #[test]
    fn lists_jwks_client_bindings_in_a_stable_order() {
        let indexed = IndexedSource::new(
            "#[provides_jwks_endpoint(partner)]\nstruct PartnerEndpoint;\n\n#[provides_jwks_endpoint(auth)]\nstruct AuthEndpoint;\n",
        );
        let pool = TagPool::collect(&indexed.index).expect("the pool collects");

        let tags: Vec<String> = pool
            .trusted_issuer_bindings()
            .iter()
            .map(|binding| binding.tag.to_string())
            .collect();

        assert_eq!(tags, vec!["auth".to_string(), "partner".to_string()]);
    }

    #[test]
    fn binds_an_oidc_issuer_tag_to_its_declaring_singleton() {
        let indexed = IndexedSource::new("#[trusts_oidc_issuer(partner)]\nstruct Issuer;\n");
        let pool = TagPool::collect(&indexed.index).expect("the pool collects");
        let bindings = pool.trusted_issuer_bindings();

        assert_eq!(bindings.len(), 1);
        assert_eq!(bindings[0].tag.to_string(), "partner");
        assert_eq!(bindings[0].declaring.to_string(), "crate::Issuer");
        assert_eq!(bindings[0].kind, TrustedIssuerKind::OidcIssuer);
        assert_eq!(bindings[0].module_segment, "issuer");
    }

    #[test]
    fn gives_same_named_issuers_of_different_modules_distinct_segments() {
        let indexed = IndexedSource::new(
            "mod first {\n    #[trusts_oidc_issuer(first)]\n    pub struct Issuer;\n}\n\nmod second {\n    #[trusts_oidc_issuer(second)]\n    pub struct Issuer;\n}\n",
        );
        let pool = TagPool::collect(&indexed.index).expect("the pool collects");

        let segments: Vec<String> = pool
            .trusted_issuer_bindings()
            .iter()
            .map(|binding| binding.module_segment.clone())
            .collect();

        assert_eq!(
            segments,
            vec!["first_issuer".to_string(), "second_issuer".to_string()]
        );
    }

    #[test]
    fn excludes_a_middleware_tag_from_trusted_issuer_bindings() {
        let indexed = IndexedSource::new(
            "#[handles_middleware_attribute(attribute = logged)]\nstruct RequestLog;\n",
        );
        let pool = TagPool::collect(&indexed.index).expect("the pool collects");

        assert!(pool.trusted_issuer_bindings().is_empty());
    }

    #[test]
    fn resolves_a_middleware_tag_to_its_handler() {
        let indexed = IndexedSource::new(
            "#[handles_middleware_attribute(attribute = logged)]\nstruct RequestLog;\n",
        );
        let pool = TagPool::collect(&indexed.index).expect("the pool collects");

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
        let indexed = IndexedSource::new(
            "#[handles_middleware_attribute(attribute = traced)]\nstruct Tracer;\n\n#[handles_middleware_attribute(attribute = logged)]\nstruct RequestLog;\n\n#[provides_jwks_endpoint(jwks)]\nstruct JwksEndpoint;\n",
        );
        let pool = TagPool::collect(&indexed.index).expect("the pool collects");
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
        let indexed = IndexedSource::new("#[provides_jwks_endpoint(jwks)]\nstruct JwksEndpoint;\n");
        let pool = TagPool::collect(&indexed.index).expect("the pool collects");

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
        let indexed = IndexedSource::new("#[trusts_oidc_issuer(partner)]\nstruct Issuer;\n");
        let pool = TagPool::collect(&indexed.index).expect("the pool collects");

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
        let indexed = IndexedSource::new("#[provides_jwks_endpoint(jwks)]\nstruct JwksEndpoint;\n");
        let pool = TagPool::collect(&indexed.index).expect("the pool collects");

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
        let indexed = IndexedSource::new("#[provides_jwks_endpoint(jwks)]\nstruct JwksEndpoint;\n");
        let pool = TagPool::collect(&indexed.index).expect("the pool collects");

        assert!(
            pool.resolve(&tag("jwks"), TagExpectation::Middleware, "the site")
                .err()
                .expect("the tag is the wrong kind")
                .to_string()
                .contains("is a jwks endpoint provider, not a middleware handler")
        );
    }

    #[test]
    fn rejects_a_tag_declared_by_both_a_jwks_endpoint_and_a_middleware() {
        assert!(
            error_for(
                "#[provides_jwks_endpoint(shared)]\nstruct Endpoint;\n\n#[handles_middleware_attribute(attribute = shared)]\nstruct Handler;\n",
            )
            .contains("declared more than once")
        );
    }

    #[test]
    fn rejects_a_jwks_endpoint_without_a_tag() {
        assert!(
            error_for("#[provides_jwks_endpoint]\nstruct JwksEndpoint;\n")
                .contains("does not name a tag")
        );
    }

    #[test]
    fn rejects_an_oidc_issuer_without_a_tag() {
        assert_eq!(
            error_for("#[trusts_oidc_issuer]\nstruct Issuer;\n"),
            "the oidc issuer 'crate::Issuer' does not name a tag"
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
    fn rejects_a_tag_that_is_not_a_plain_name() {
        assert!(
            error_for("#[provides_jwks_endpoint(endpoints::jwks)]\nstruct JwksEndpoint;\n")
                .contains("not a single plain name")
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
            error_for("#[provides_jwks_endpoint(= 5)]\nstruct JwksEndpoint;\n")
                .contains("failed to index")
        );
    }

    fn exchanger_bindings(lib_source: &str) -> Result<Vec<String>, TagError> {
        let indexed = IndexedSource::new(lib_source);
        let pool = TagPool::collect(&indexed.index).expect("the pool collects");
        let trusted_issuers = pool.trusted_issuer_bindings();

        pool.subject_token_exchanger_bindings(&indexed.index, &trusted_issuers)
            .map(|bindings| {
                bindings
                    .iter()
                    .map(|binding| format!("{}={}", binding.issuer.tag, binding.declaring))
                    .collect()
            })
    }

    #[test]
    fn binds_a_subject_token_exchanger_to_its_issuer() {
        assert_eq!(
            exchanger_bindings(
                "#[provides_jwks_endpoint(ci)]\nstruct Ci;\n\n#[exchanges_subject_tokens(issuer = ci)]\nstruct CiExchanger;\n"
            )
            .expect("the exchanger names a trusted issuer"),
            vec!["ci=crate::CiExchanger".to_string()]
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
            exchanger_bindings(
                "#[trusts_oidc_issuer(partner)]\nstruct Issuer;\n\n#[oauth_client(partner_client, issuer = partner)]\nstruct PartnerClient;\n\n#[exchanges_subject_tokens(issuer = partner_client)]\nstruct Exchanger;\n"
            )
            .expect_err("an oauth client is not a trusted issuer")
            .to_string(),
            "the subject token exchanger 'crate::Exchanger' references the tag 'partner_client', which is a oauth client, not a trusted issuer"
        );
    }

    #[test]
    fn rejects_a_second_subject_token_exchanger_of_an_issuer() {
        assert_eq!(
            exchanger_bindings(
                "#[provides_jwks_endpoint(ci)]\nstruct Ci;\n\n#[exchanges_subject_tokens(issuer = ci)]\nstruct First;\n\n#[exchanges_subject_tokens(issuer = ci)]\nstruct Second;\n"
            )
            .expect_err("an issuer has one exchanger")
            .to_string(),
            "the issuer 'ci' has more than one subject token exchanger: 'crate::First' and 'crate::Second'"
        );
    }
}
