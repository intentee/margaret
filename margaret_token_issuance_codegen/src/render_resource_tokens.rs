use quote::ToTokens as _;
use quote::quote;
use syn::ext::IdentExt as _;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::declared_resource_issuances::DeclaredResourceIssuances;
use crate::resource_issuance_declaration::ResourceIssuanceDeclaration;
use crate::resource_tokens_module_name::RESOURCE_TOKENS_MODULE_NAME;

fn resource_module(
    ResourceIssuanceDeclaration { audience, tag, .. }: &ResourceIssuanceDeclaration,
) -> GeneratedModuleTokens {
    let audience = audience.as_str();

    GeneratedModuleTokens::new(
        format!("{RESOURCE_TOKENS_MODULE_NAME}/{}", tag.ident().unraw()),
        quote! { pub const AUDIENCE: &str = #audience; },
    )
}

#[must_use]
pub fn render_resource_tokens(resources: &DeclaredResourceIssuances) -> Vec<GeneratedModuleTokens> {
    if resources.resources().next().is_none() {
        return Vec::new();
    }

    let submodules = resources.resources().map(|resource| {
        let module = resource.tag.ident().to_token_stream();

        quote! { pub mod #module; }
    });

    [GeneratedModuleTokens::new(
        RESOURCE_TOKENS_MODULE_NAME,
        quote! { #(#submodules)* },
    )]
    .into_iter()
    .chain(resources.resources().map(resource_module))
    .collect()
}

#[cfg(test)]
mod tests {
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

    use super::render_resource_tokens;
    use crate::declared_resource_issuances::DeclaredResourceIssuances;
    use crate::declared_token_issuance::DeclaredTokenIssuance;

    fn rendered(source: &str) -> Vec<GeneratedModuleTokens> {
        let indexed = IndexedSource::new(source);
        let issuance =
            DeclaredTokenIssuance::read(&indexed.index).expect("the token issuance is read");

        render_resource_tokens(
            &DeclaredResourceIssuances::read(&indexed.index, &issuance)
                .expect("the resources are read"),
        )
    }

    fn sources(modules: Vec<GeneratedModuleTokens>) -> Vec<String> {
        modules
            .into_iter()
            .map(|module| {
                let formatted = module.format().expect("the module formats");

                format!("{}: {}", formatted.name(), formatted.source())
            })
            .collect()
    }

    #[test]
    fn renders_nothing_without_resources() {
        assert!(rendered("pub struct Nothing;\n").is_empty());
    }

    #[test]
    fn renders_the_audience_of_each_resource() {
        assert_eq!(
            sources(rendered(
                "#[issues_tokens(provider, audience = \"session\", issuer = \"https://issuer.example\")]\npub struct Issuer;\n#[issues_resource_tokens(r#async, audience = \"jobs\")]\npub struct Jobs;\n#[issues_resource_tokens(attachments, audience = \"files\")]\npub struct Attachments;\n"
            )),
            vec![
                "resource_tokens: pub mod attachments;\npub mod r#async;\n".to_string(),
                "resource_tokens/attachments: pub const AUDIENCE: &str = \"files\";\n".to_string(),
                "resource_tokens/async: pub const AUDIENCE: &str = \"jobs\";\n".to_string(),
            ]
        );
    }
}
