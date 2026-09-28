use std::collections::HashMap;

use syn::Path;

use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::field_base::field_base;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::name_allocator::NameAllocator;
use margaret_attributes::tag::Tag;

use crate::jwks_client_binding::JwksClientBinding;
use crate::read_middleware_attribute::read_middleware_attribute;
use crate::tag_error::TagError;
use crate::tag_kind::TagKind;
use crate::tagged_item::TaggedItem;

struct TagEntry<'index> {
    item: &'index IndexedItem,
    kind: TagKind,
}

fn collect_kind<'index>(
    index: &'index AttributeIndex,
    framework_attribute: FrameworkAttribute,
    kind: TagKind,
    entries: &mut HashMap<Tag, TagEntry<'index>>,
) -> Result<(), TagError> {
    for matched in index.select_framework_attribute(framework_attribute) {
        let item = matched.item();
        let declared = declared_tag_path(matched.args()?, kind)?;
        let path = declared.ok_or_else(|| TagError::MissingTag {
            concrete: item.canonical_path().to_string(),
            kind,
        })?;
        let tag = Tag::from_path(&path).ok_or_else(|| TagError::MalformedTag {
            concrete: item.canonical_path().to_string(),
            kind,
        })?;

        if let Some(existing) = entries.get(&tag) {
            return Err(TagError::DuplicateTag {
                tag: tag.to_string(),
                first: existing.item.canonical_path().to_string(),
                second: item.canonical_path().to_string(),
            });
        }

        entries.insert(tag, TagEntry { item, kind });
    }

    Ok(())
}

fn declared_tag_path(args: &AttributeArgs, kind: TagKind) -> Result<Option<Path>, TagError> {
    match kind {
        TagKind::JwksClient => {
            args.interpret(|reader| Ok::<_, TagError>(reader.take_positional_path()))
        }
        TagKind::Middleware => Ok(read_middleware_attribute(args)?),
    }
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

        collect_kind(
            index,
            FrameworkAttribute::ProvidesJwksEndpoint,
            TagKind::JwksClient,
            &mut entries,
        )?;
        collect_kind(
            index,
            FrameworkAttribute::HandlesMiddlewareAttribute,
            TagKind::Middleware,
            &mut entries,
        )?;

        Ok(Self { entries })
    }

    #[must_use]
    pub fn jwks_client_bindings(&self) -> Vec<JwksClientBinding> {
        let mut allocator = NameAllocator::new();

        self.tagged_items(TagKind::JwksClient)
            .into_iter()
            .map(|TaggedItem { item, tag }| JwksClientBinding {
                endpoint: item.canonical_path().clone(),
                module_segment: allocator
                    .allocate(&field_base(item.canonical_path()))
                    .field()
                    .to_string(),
                tag,
            })
            .collect()
    }

    #[must_use]
    pub fn middleware_handlers(&self) -> Vec<TaggedItem<'index>> {
        let mut handlers = self.tagged_items(TagKind::Middleware);

        handlers.sort_by(|left, right| left.item.canonical_path().cmp(right.item.canonical_path()));

        handlers
    }

    /// # Errors
    ///
    /// Returns `TagError::UnknownTag` or `TagError::WrongKind`.
    pub fn resolve(
        &self,
        tag: &Tag,
        expected: TagKind,
        site: &str,
    ) -> Result<&'index IndexedItem, TagError> {
        match self.entries.get(tag) {
            None => Err(TagError::UnknownTag {
                site: site.to_string(),
                tag: tag.to_string(),
                kind: expected,
            }),
            Some(entry) if entry.kind == expected => Ok(entry.item),
            Some(entry) => Err(TagError::WrongKind {
                site: site.to_string(),
                tag: tag.to_string(),
                expected,
                found: entry.kind,
            }),
        }
    }

    fn tagged_items(&self, kind: TagKind) -> Vec<TaggedItem<'index>> {
        let mut tagged: Vec<TaggedItem<'index>> = self
            .entries
            .iter()
            .filter(|(_, entry)| entry.kind == kind)
            .map(|(tag, entry)| TaggedItem {
                item: entry.item,
                tag: tag.clone(),
            })
            .collect();

        tagged.sort_by_key(|tagged_item| tagged_item.tag.to_string());

        tagged
    }
}

#[cfg(test)]
mod tests {
    use syn::Path;
    use syn::parse_str;

    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_attributes::tag::Tag;
    use margaret_attributes_tests::indexed_source::IndexedSource;

    use crate::tag_error::TagError;
    use crate::tag_kind::TagKind;
    use crate::tag_pool::TagPool;

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

    #[test]
    fn binds_a_jwks_client_tag_to_its_endpoint() {
        let indexed = IndexedSource::new("#[provides_jwks_endpoint(jwks)]\nstruct JwksEndpoint;\n");
        let pool = TagPool::collect(&indexed.index).expect("the pool collects");
        let bindings = pool.jwks_client_bindings();

        assert_eq!(bindings.len(), 1);
        assert_eq!(bindings[0].tag.to_string(), "jwks");
        assert_eq!(bindings[0].endpoint.to_string(), "crate::JwksEndpoint");
        assert_eq!(bindings[0].module_segment, "jwks_endpoint");
    }

    #[test]
    fn takes_a_raw_identifier_tag_verbatim_and_derives_the_segment_from_the_endpoint() {
        let indexed =
            IndexedSource::new("#[provides_jwks_endpoint(r#async)]\nstruct AsyncEndpoint;\n");
        let pool = TagPool::collect(&indexed.index).expect("the pool collects");
        let bindings = pool.jwks_client_bindings();

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
            .jwks_client_bindings()
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
            .jwks_client_bindings()
            .iter()
            .map(|binding| binding.tag.to_string())
            .collect();

        assert_eq!(tags, vec!["auth".to_string(), "partner".to_string()]);
    }

    #[test]
    fn excludes_a_middleware_tag_from_jwks_client_bindings() {
        let indexed = IndexedSource::new(
            "#[handles_middleware_attribute(attribute = logged)]\nstruct RequestLog;\n",
        );
        let pool = TagPool::collect(&indexed.index).expect("the pool collects");

        assert!(pool.jwks_client_bindings().is_empty());
    }

    #[test]
    fn resolves_a_middleware_tag_to_its_handler() {
        let indexed = IndexedSource::new(
            "#[handles_middleware_attribute(attribute = logged)]\nstruct RequestLog;\n",
        );
        let pool = TagPool::collect(&indexed.index).expect("the pool collects");

        assert_eq!(
            pool.resolve(&tag("logged"), TagKind::Middleware, "the site")
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
    fn reports_an_unknown_tag() {
        let indexed = IndexedSource::new("#[provides_jwks_endpoint(jwks)]\nstruct JwksEndpoint;\n");
        let pool = TagPool::collect(&indexed.index).expect("the pool collects");

        assert!(
            pool.resolve(&tag("missing"), TagKind::JwksClient, "the site")
                .err()
                .expect("the tag is unknown")
                .to_string()
                .contains("no jwks endpoint provider declares")
        );
    }

    #[test]
    fn reports_a_tag_of_the_wrong_kind() {
        let indexed = IndexedSource::new("#[provides_jwks_endpoint(jwks)]\nstruct JwksEndpoint;\n");
        let pool = TagPool::collect(&indexed.index).expect("the pool collects");

        assert!(
            pool.resolve(&tag("jwks"), TagKind::Middleware, "the site")
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
}
