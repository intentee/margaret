use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::token_issuance_declaration::TokenIssuanceDeclaration;
use crate::token_issuance_module_name::TOKEN_ISSUANCE_MODULE_NAME;

#[must_use]
pub fn render_token_issuance(
    TokenIssuanceDeclaration {
        audience, issuer, ..
    }: &TokenIssuanceDeclaration,
) -> GeneratedModuleTokens {
    let audience = audience.as_str();
    let issuer = issuer.as_str();

    GeneratedModuleTokens::new(
        TOKEN_ISSUANCE_MODULE_NAME,
        quote! {
            pub const TOKEN_ISSUANCE: margaret::framework::token_issuance::token_issuance::TokenIssuance =
                margaret::framework::token_issuance::token_issuance::TokenIssuance {
                    audience: #audience,
                    issuer: #issuer,
                };
        },
    )
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;

    use super::render_token_issuance;
    use crate::token_issuance_declaration::TokenIssuanceDeclaration;

    #[test]
    fn renders_the_token_issuance_as_a_constant() {
        let module = render_token_issuance(&TokenIssuanceDeclaration {
            anchor: CanonicalPath::new(vec!["crate".to_string(), "Issuer".to_string()]),
            audience: "session".parse().expect("the audience is not empty"),
            issuer: "https://issuer.example"
                .parse()
                .expect("the issuer is an https url"),
        })
        .format()
        .expect("the module formats");

        assert_eq!(module.name(), "token_issuance");
        assert_eq!(
            module.source(),
            "pub const TOKEN_ISSUANCE: margaret::framework::token_issuance::token_issuance::TokenIssuance = margaret::framework::token_issuance::token_issuance::TokenIssuance {\n    audience: \"session\",\n    issuer: \"https://issuer.example\",\n};\n"
        );
    }
}
