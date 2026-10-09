use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::declared_scope_item::DeclaredScopeItem;
use crate::declared_scopes::DeclaredScopes;
use crate::scopes_module_name::SCOPES_MODULE_NAME;

#[must_use]
pub fn render_scopes(DeclaredScopes { scopes }: &DeclaredScopes) -> Vec<GeneratedModuleTokens> {
    if scopes.is_empty() {
        return Vec::new();
    }

    let implementations = scopes.iter().map(|DeclaredScopeItem { anchor, scope }| {
        let declaring = path_tokens(anchor.canonical_path());
        let name = scope.as_str();

        quote! {
            impl margaret::framework::oauth_vocabulary::declared_scope::DeclaredScope for #declaring {
                const NAME: &'static str = #name;
            }
        }
    });

    vec![GeneratedModuleTokens::new(
        SCOPES_MODULE_NAME,
        quote! { #(#implementations)* },
    )]
}

#[cfg(test)]
mod tests {
    use margaret_attributes_tests::indexed_source::IndexedSource;

    use super::render_scopes;
    use crate::declared_scopes::DeclaredScopes;

    fn rendered(source: &str) -> Vec<String> {
        let indexed = IndexedSource::new(source);

        render_scopes(&DeclaredScopes::read(&indexed.index).expect("the scopes are read"))
            .into_iter()
            .map(|module| module.to_source().split_whitespace().collect())
            .collect()
    }

    #[test]
    fn implements_the_declared_scope_of_each_scope_item() {
        assert_eq!(
            rendered("#[oauth_scope(name = \"profile\")]\npub struct ProfileScope;\n"),
            vec![
                "implmargaret::framework::oauth_vocabulary::declared_scope::DeclaredScopeforcrate::ProfileScope{constNAME:&'staticstr=\"profile\";}".to_string()
            ]
        );
    }

    #[test]
    fn renders_no_module_without_declared_scopes() {
        assert!(rendered("pub struct Unrelated;\n").is_empty());
    }
}
