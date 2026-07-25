use std::collections::HashMap;

use syn::Path;

use margaret_attributes::attribute_args::AttributeArgs;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::tag::Tag;

use crate::jwks_client_binding::JwksClientBinding;
use crate::read_middleware_attribute::read_middleware_attribute;
use crate::tag_error::TagError;
use crate::tag_kind::TagKind;

struct TagEntry {
    concrete: CanonicalPath,
    kind: TagKind,
}

fn collect_kind(
    index: &AttributeIndex,
    selector: &AttributeSelector,
    kind: TagKind,
    entries: &mut HashMap<Tag, TagEntry>,
) -> Result<(), TagError> {
    for matched in index.select(selector) {
        let concrete = matched.item().canonical_path().clone();
        let declared = declared_tag_path(matched.args()?, kind)?;
        let path = declared.ok_or_else(|| TagError::MissingTag {
            concrete: concrete.to_string(),
            kind,
        })?;
        let tag = Tag::from_path(&path).ok_or_else(|| TagError::MalformedTag {
            concrete: concrete.to_string(),
            kind,
        })?;

        if let Some(existing) = entries.get(&tag) {
            return Err(TagError::DuplicateTag {
                tag: tag.to_string(),
                first: existing.concrete.to_string(),
                second: concrete.to_string(),
            });
        }

        entries.insert(tag, TagEntry { concrete, kind });
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

fn jwks_endpoint_selector() -> AttributeSelector {
    AttributeSelector::from_marker("provides_jwks_endpoint")
}

fn middleware_selector() -> AttributeSelector {
    AttributeSelector::from_marker("handles_middleware_attribute")
}

pub struct TagPool {
    entries: HashMap<Tag, TagEntry>,
}

impl TagPool {
    pub fn collect(index: &AttributeIndex) -> Result<Self, TagError> {
        let mut entries: HashMap<Tag, TagEntry> = HashMap::new();

        collect_kind(
            index,
            &jwks_endpoint_selector(),
            TagKind::JwksClient,
            &mut entries,
        )?;
        collect_kind(
            index,
            &middleware_selector(),
            TagKind::Middleware,
            &mut entries,
        )?;

        Ok(Self { entries })
    }

    #[must_use]
    pub fn jwks_client_bindings(&self) -> Vec<JwksClientBinding> {
        let mut bindings: Vec<JwksClientBinding> = self
            .entries
            .iter()
            .filter(|(_, entry)| entry.kind == TagKind::JwksClient)
            .map(|(tag, entry)| JwksClientBinding {
                endpoint: entry.concrete.clone(),
                tag: tag.clone(),
            })
            .collect();

        bindings.sort_by_key(|binding| binding.tag.to_string());

        bindings
    }

    pub fn resolve(
        &self,
        tag: &Tag,
        expected: TagKind,
        site: &str,
    ) -> Result<&CanonicalPath, TagError> {
        match self.entries.get(tag) {
            None => Err(TagError::UnknownTag {
                site: site.to_string(),
                tag: tag.to_string(),
                kind: expected,
            }),
            Some(entry) if entry.kind == expected => Ok(&entry.concrete),
            Some(entry) => Err(TagError::WrongKind {
                site: site.to_string(),
                tag: tag.to_string(),
                expected,
                found: entry.kind,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use syn::Path;
    use syn::parse_str;
    use tempfile::tempdir;

    use margaret_attributes::attribute_index::AttributeIndex;
    use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
    use margaret_attributes::crate_root::CrateRoot;
    use margaret_attributes::tag::Tag;

    use crate::tag_error::TagError;
    use crate::tag_kind::TagKind;
    use crate::tag_pool::TagPool;

    fn index_for(lib_source: &str) -> AttributeIndex {
        let directory = tempdir().expect("a temporary crate directory is created");
        let source_directory = directory.path().join("src");

        fs::create_dir(&source_directory).expect("the src directory is created");
        fs::write(source_directory.join("lib.rs"), lib_source).expect("lib.rs is written");

        AttributeIndexBuilder::new()
            .index_crate(&CrateRoot::new("crate", source_directory))
            .expect("the crate is indexed")
            .build()
    }

    fn pool_for(lib_source: &str) -> Result<TagPool, TagError> {
        TagPool::collect(&index_for(lib_source))
    }

    fn tag(name: &str) -> Tag {
        let path: Path = parse_str(name).expect("the tag path parses");

        Tag::from_path(&path).expect("the tag is a plain name")
    }

    fn error_for(lib_source: &str) -> String {
        pool_for(lib_source)
            .err()
            .expect("the pool fails to collect")
            .to_string()
    }

    #[test]
    fn binds_a_jwks_client_tag_to_its_endpoint() {
        let pool = pool_for("#[provides_jwks_endpoint(jwks)]\nstruct JwksEndpoint;\n")
            .expect("the pool collects");
        let bindings = pool.jwks_client_bindings();

        assert_eq!(bindings.len(), 1);
        assert_eq!(bindings[0].tag.to_string(), "jwks");
        assert_eq!(bindings[0].endpoint.to_string(), "crate::JwksEndpoint");
    }

    #[test]
    fn lists_jwks_client_bindings_in_a_stable_order() {
        let pool = pool_for(
            "#[provides_jwks_endpoint(partner)]\nstruct PartnerEndpoint;\n\n#[provides_jwks_endpoint(auth)]\nstruct AuthEndpoint;\n",
        )
        .expect("the pool collects");

        let tags: Vec<String> = pool
            .jwks_client_bindings()
            .iter()
            .map(|binding| binding.tag.to_string())
            .collect();

        assert_eq!(tags, vec!["auth".to_string(), "partner".to_string()]);
    }

    #[test]
    fn excludes_a_middleware_tag_from_jwks_client_bindings() {
        let pool =
            pool_for("#[handles_middleware_attribute(attribute = logged)]\nstruct RequestLog;\n")
                .expect("the pool collects");

        assert!(pool.jwks_client_bindings().is_empty());
    }

    #[test]
    fn resolves_a_middleware_tag_to_its_handler() {
        let pool =
            pool_for("#[handles_middleware_attribute(attribute = logged)]\nstruct RequestLog;\n")
                .expect("the pool collects");

        assert_eq!(
            pool.resolve(&tag("logged"), TagKind::Middleware, "the site")
                .expect("the tag resolves")
                .to_string(),
            "crate::RequestLog"
        );
    }

    #[test]
    fn reports_an_unknown_tag() {
        let pool = pool_for("#[provides_jwks_endpoint(jwks)]\nstruct JwksEndpoint;\n")
            .expect("the pool collects");

        assert!(
            pool.resolve(&tag("missing"), TagKind::JwksClient, "the site")
                .expect_err("the tag is unknown")
                .to_string()
                .contains("no jwks endpoint provider declares")
        );
    }

    #[test]
    fn reports_a_tag_of_the_wrong_kind() {
        let pool = pool_for("#[provides_jwks_endpoint(jwks)]\nstruct JwksEndpoint;\n")
            .expect("the pool collects");

        assert!(
            pool.resolve(&tag("jwks"), TagKind::Middleware, "the site")
                .expect_err("the tag is the wrong kind")
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
    fn reports_a_non_path_middleware_tag_argument() {
        assert!(
            error_for(
                "#[handles_middleware_attribute(attribute = \"logged\")]\nstruct RequestLog;\n"
            )
            .contains("failed to index")
        );
    }

    #[test]
    fn reports_unparseable_attribute_arguments() {
        assert!(
            error_for("#[provides_jwks_endpoint(= 5)]\nstruct JwksEndpoint;\n")
                .contains("failed to index")
        );
    }
}
