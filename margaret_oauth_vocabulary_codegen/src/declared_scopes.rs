use std::collections::BTreeSet;

use syn::Path;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::matched_attribute::MatchedAttribute;
use margaret_declaration_anchor::declaration_anchor::declaration_anchor;
use margaret_oauth_vocabulary::scope::Scope;
use margaret_oauth_vocabulary::scope_parsing::ScopeParsing;

use crate::declared_scope_item::DeclaredScopeItem;
use crate::oauth_vocabulary_codegen_error::OAuthVocabularyCodegenError;
use crate::scope_resolution::ScopeResolution;

fn openid_scope_path() -> CanonicalPath {
    CanonicalPath::new(
        [
            "margaret",
            "framework",
            "oauth_vocabulary",
            "openid_scope",
            "OpenidScope",
        ]
        .into_iter()
        .map(ToString::to_string)
        .collect(),
    )
}

fn declared_scope<'index>(
    index: &'index AttributeIndex,
    matched: &MatchedAttribute<'index>,
) -> Result<DeclaredScopeItem<'index>, OAuthVocabularyCodegenError> {
    let anchor = declaration_anchor(index, matched, FrameworkAttribute::OAuthScope)?.item;
    let path = anchor.canonical_path().to_string();
    let name = matched.args()?.interpret(|reader| {
        reader
            .take_string("name")?
            .ok_or_else(|| OAuthVocabularyCodegenError::MissingScopeName {
                anchor: path.clone(),
            })
    })?;
    let scope = match Scope::parse(&name) {
        ScopeParsing::Accepted(scope) => scope,
        ScopeParsing::Rejected(rejection) => {
            return Err(OAuthVocabularyCodegenError::MalformedScopeName {
                anchor: path,
                rejection,
            });
        }
    };

    if scope.is_openid() {
        return Err(OAuthVocabularyCodegenError::RedeclaredOpenidScope { anchor: path });
    }

    Ok(DeclaredScopeItem { anchor, scope })
}

pub struct DeclaredScopes<'index> {
    pub scopes: Vec<DeclaredScopeItem<'index>>,
}

impl<'index> DeclaredScopes<'index> {
    /// # Errors
    ///
    /// Returns `OAuthVocabularyCodegenError` when an `#[oauth_scope]` declaration is not anchored
    /// by a plain struct, names no valid scope, redeclares the openid scope, or declares a scope
    /// another declaration already declares.
    pub fn read(index: &'index AttributeIndex) -> Result<Self, OAuthVocabularyCodegenError> {
        let mut scopes: Vec<DeclaredScopeItem<'index>> = Vec::new();

        for matched in index.select_framework_attribute(FrameworkAttribute::OAuthScope) {
            let declared = declared_scope(index, &matched)?;

            if let Some(first) = scopes
                .iter()
                .find(|existing| existing.scope == declared.scope)
            {
                return Err(OAuthVocabularyCodegenError::DuplicateScopeName {
                    first: first.anchor.canonical_path().to_string(),
                    name: declared.scope.as_str().to_string(),
                    second: declared.anchor.canonical_path().to_string(),
                });
            }

            scopes.push(declared);
        }

        Ok(Self { scopes })
    }

    #[must_use]
    pub fn resolve(
        &self,
        index: &AttributeIndex,
        item: &IndexedItem,
        written: &Path,
    ) -> ScopeResolution {
        let Some(resolved) = index.resolve_item_path(item, written) else {
            return ScopeResolution::Unknown;
        };

        if resolved == openid_scope_path() {
            return ScopeResolution::Declared(Scope::openid());
        }

        self.scopes
            .iter()
            .find(|declared| *declared.anchor.canonical_path() == resolved)
            .map_or(ScopeResolution::Unknown, |declared| {
                ScopeResolution::Declared(declared.scope.clone())
            })
    }

    /// # Errors
    ///
    /// Returns `OAuthVocabularyCodegenError::UnconsumedScope` for the first declared scope that
    /// no client declaration references.
    pub fn reject_unconsumed(
        &self,
        referenced: &BTreeSet<&str>,
    ) -> Result<(), OAuthVocabularyCodegenError> {
        match self
            .scopes
            .iter()
            .find(|declared| !referenced.contains(declared.scope.as_str()))
        {
            Some(declared) => Err(OAuthVocabularyCodegenError::UnconsumedScope {
                anchor: declared.anchor.canonical_path().to_string(),
                name: declared.scope.as_str().to_string(),
            }),
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use syn::parse_quote;

    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_attributes::attribute_error::AttributeError;
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_declaration_anchor::declaration_anchor_error::DeclarationAnchorError;

    use super::DeclaredScopes;
    use crate::oauth_vocabulary_codegen_error::OAuthVocabularyCodegenError;
    use crate::scope_resolution::ScopeResolution;

    const PROFILE: &str = "#[oauth_scope(name = \"profile\")]\npub struct ProfileScope;\n";

    fn read(source: &str) -> Result<Vec<String>, OAuthVocabularyCodegenError> {
        DeclaredScopes::read(&IndexedSource::new(source).index).map(|declared| {
            declared
                .scopes
                .iter()
                .map(|scope| format!("{}={}", scope.anchor.canonical_path(), scope.scope.as_str()))
                .collect()
        })
    }

    fn resolved(source: &str, written: &syn::Path) -> ScopeResolution {
        let indexed = IndexedSource::new(source);
        let declared = DeclaredScopes::read(&indexed.index).expect("the scopes are read");
        let item = indexed
            .index
            .item(&CanonicalPath::new(vec![
                "crate".to_string(),
                "Client".to_string(),
            ]))
            .expect("the client is indexed");

        declared.resolve(&indexed.index, item, written)
    }

    #[test]
    fn reads_a_declared_scope() {
        assert_eq!(
            read(PROFILE).expect("the scope is read"),
            vec!["crate::ProfileScope=profile".to_string()]
        );
    }

    #[test]
    fn rejects_a_declaration_without_a_name() {
        assert!(matches!(
            read("#[oauth_scope]\npub struct ProfileScope;\n"),
            Err(OAuthVocabularyCodegenError::MissingScopeName { anchor }) if anchor == "crate::ProfileScope"
        ));
    }

    #[test]
    fn rejects_a_scope_name_that_is_not_a_string() {
        assert!(matches!(
            read("#[oauth_scope(name = profile)]\npub struct ProfileScope;\n"),
            Err(OAuthVocabularyCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            )) if key == "name"
        ));
    }

    #[test]
    fn rejects_arguments_that_do_not_parse() {
        assert!(matches!(
            read("#[oauth_scope(= 5)]\npub struct ProfileScope;\n"),
            Err(OAuthVocabularyCodegenError::Index(AttributeError::Arguments(
                AttributeArgumentsError::Malformed { attribute_path, .. }
            ))) if attribute_path == "oauth_scope"
        ));
    }

    #[test]
    fn rejects_a_scope_declared_on_a_singleton() {
        assert!(matches!(
            read("#[singleton]\n#[oauth_scope(name = \"profile\")]\npub struct ProfileScope;\n"),
            Err(OAuthVocabularyCodegenError::Anchor(DeclarationAnchorError::DeclaredAsSingleton { path, .. }))
                if path == "crate::ProfileScope"
        ));
    }

    #[test]
    fn rejects_a_malformed_scope_name() {
        assert!(matches!(
            read("#[oauth_scope(name = \"two words\")]\npub struct ProfileScope;\n"),
            Err(OAuthVocabularyCodegenError::MalformedScopeName { anchor, .. }) if anchor == "crate::ProfileScope"
        ));
    }

    #[test]
    fn rejects_a_scope_declared_twice() {
        assert!(matches!(
            read(&format!("{PROFILE}#[oauth_scope(name = \"profile\")]\npub struct OtherProfileScope;\n")),
            Err(OAuthVocabularyCodegenError::DuplicateScopeName { name, .. }) if name == "profile"
        ));
    }

    #[test]
    fn rejects_a_redeclared_openid_scope() {
        assert!(matches!(
            read("#[oauth_scope(name = \"openid\")]\npub struct OpenidScope;\n"),
            Err(OAuthVocabularyCodegenError::RedeclaredOpenidScope { anchor }) if anchor == "crate::OpenidScope"
        ));
    }

    #[test]
    fn resolves_a_declared_scope_imported_through_a_use_statement() {
        assert!(matches!(
            resolved(
                "mod auth {\n    #[oauth_scope(name = \"profile\")]\n    pub struct ProfileScope;\n}\nuse crate::auth::ProfileScope;\npub struct Client;\n",
                &parse_quote!(ProfileScope),
            ),
            ScopeResolution::Declared(scope) if scope.as_str() == "profile"
        ));
    }

    #[test]
    fn resolves_the_framework_openid_scope() {
        assert!(matches!(
            resolved(
                "use margaret::framework::oauth_vocabulary::openid_scope::OpenidScope;\npub struct Client;\n",
                &parse_quote!(OpenidScope),
            ),
            ScopeResolution::Declared(scope) if scope.as_str() == "openid"
        ));
    }

    #[test]
    fn resolves_no_scope_for_an_undeclared_item() {
        assert_eq!(
            resolved(
                &format!("{PROFILE}pub struct Unrelated;\npub struct Client;\n"),
                &parse_quote!(Unrelated),
            ),
            ScopeResolution::Unknown
        );
    }

    #[test]
    fn resolves_no_scope_for_an_unresolvable_path() {
        assert_eq!(
            resolved("pub struct Client;\n", &parse_quote!(missing::Scope)),
            ScopeResolution::Unknown
        );
    }

    #[test]
    fn rejects_a_declared_scope_nothing_references() {
        let indexed = IndexedSource::new(PROFILE);

        assert!(matches!(
            DeclaredScopes::read(&indexed.index)
                .expect("the scopes are read")
                .reject_unconsumed(&BTreeSet::from(["openid"])),
            Err(OAuthVocabularyCodegenError::UnconsumedScope { name, .. }) if name == "profile"
        ));
    }

    #[test]
    fn accepts_declared_scopes_that_are_referenced() {
        let indexed = IndexedSource::new(PROFILE);

        assert!(
            DeclaredScopes::read(&indexed.index)
                .expect("the scopes are read")
                .reject_unconsumed(&BTreeSet::from(["profile"]))
                .is_ok()
        );
    }
}
