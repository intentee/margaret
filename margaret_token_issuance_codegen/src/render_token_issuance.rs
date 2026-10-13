use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_generated_module::module_path::ModulePath;

use crate::constants::TOKEN_ISSUANCE_MODULE_NAME;
use crate::token_issuance_declaration::TokenIssuanceDeclaration;

#[must_use]
pub fn render_token_issuance(
    TokenIssuanceDeclaration { issuer, .. }: &TokenIssuanceDeclaration,
) -> GeneratedModuleTokens {
    let issuer = issuer.as_str();

    GeneratedModuleTokens::new(
        ModulePath::root(TOKEN_ISSUANCE_MODULE_NAME),
        quote! {
            pub const TOKEN_ISSUANCE: margaret::framework::token_issuance::token_issuance::TokenIssuance =
                margaret::framework::token_issuance::token_issuance::TokenIssuance {
                    issuer: #issuer,
                };
        },
    )
}

#[cfg(test)]
mod tests {
    use margaret_attributes_tests::indexed_source::IndexedSource;

    use super::render_token_issuance;
    use crate::declared_token_issuance::DeclaredTokenIssuance;

    #[test]
    fn renders_the_token_issuance_as_a_constant() {
        let indexed = IndexedSource::new(
            "use margaret::framework::macros::issues_tokens;\n#[issues_tokens(provider, issuer = \"https://issuer.example\")]\npub struct Issuer;\n",
        );
        let declared =
            DeclaredTokenIssuance::read(&indexed.index).expect("the token issuance is read");

        assert!(matches!(
            &declared,
            DeclaredTokenIssuance::Declared(issuance)
                if render_token_issuance(issuance).format().is_ok_and(|module| {
                    module.path().to_string() == "token_issuance"
                        && module.source()
                            == "pub const TOKEN_ISSUANCE: margaret::framework::token_issuance::token_issuance::TokenIssuance = margaret::framework::token_issuance::token_issuance::TokenIssuance {\n    issuer: \"https://issuer.example\",\n};\n"
                })
        ));
    }
}
