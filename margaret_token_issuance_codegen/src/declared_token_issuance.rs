use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_declaration_anchor::declaration_anchor::declaration_anchor;

use crate::token_issuance_codegen_error::TokenIssuanceCodegenError;
use crate::token_issuance_declaration::TokenIssuanceDeclaration;

#[derive(Debug, Eq, PartialEq)]
pub enum DeclaredTokenIssuance {
    Absent,
    Declared(TokenIssuanceDeclaration),
}

impl DeclaredTokenIssuance {
    /// # Errors
    ///
    /// Returns `TokenIssuanceCodegenError` when a declaration is malformed or when more than one
    /// struct declares the token issuance.
    pub fn read(index: &AttributeIndex) -> Result<Self, TokenIssuanceCodegenError> {
        let mut declarations = Vec::new();

        for matched in index.select_framework_attribute(FrameworkAttribute::IssuesTokens) {
            let anchor = declaration_anchor(index, &matched, FrameworkAttribute::IssuesTokens)?;

            declarations.push(TokenIssuanceDeclaration::read(
                &matched,
                anchor.item.canonical_path(),
            )?);
        }

        let mut declarations = declarations.into_iter();
        let Some(declared) = declarations.next() else {
            return Ok(Self::Absent);
        };

        match declarations.next() {
            Some(another) => Err(TokenIssuanceCodegenError::AmbiguousTokenIssuance {
                first: declared.anchor.to_string(),
                second: another.anchor.to_string(),
            }),
            None => Ok(Self::Declared(declared)),
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_attributes::attribute_error::AttributeError;
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_declaration_anchor::declaration_anchor_error::DeclarationAnchorError;

    use super::DeclaredTokenIssuance;
    use crate::token_issuance_codegen_error::TokenIssuanceCodegenError;
    use crate::token_issuance_declaration::TokenIssuanceDeclaration;

    fn read(source: &str) -> Result<DeclaredTokenIssuance, TokenIssuanceCodegenError> {
        DeclaredTokenIssuance::read(&IndexedSource::new(source).index)
    }

    fn rejection(arguments: &str) -> TokenIssuanceCodegenError {
        read(&format!(
            "#[issues_tokens({arguments})]\npub struct Issuer;\n"
        ))
        .expect_err("the declaration is rejected")
    }

    #[test]
    fn reads_the_declared_audience_and_issuer() {
        assert_eq!(
            read(
                "#[issues_tokens(audience = \"session\", issuer = \"https://issuer.example\")]\npub struct Issuer;\n",
            )
            .expect("the token issuance is read"),
            DeclaredTokenIssuance::Declared(TokenIssuanceDeclaration {
                anchor: CanonicalPath::new(vec!["crate".to_string(), "Issuer".to_string()]),
                audience: "session".parse().expect("the audience is valid"),
                issuer: "https://issuer.example"
                    .parse()
                    .expect("the issuer is valid"),
            })
        );
    }

    #[test]
    fn reads_no_issuance_when_nothing_declares_it() {
        assert_eq!(
            read("pub struct Issuer;\n").expect("the absent issuance is read"),
            DeclaredTokenIssuance::Absent
        );
    }

    #[test]
    fn rejects_two_declarations_of_the_token_issuance() {
        assert!(matches!(
            read(
                "#[issues_tokens(audience = \"a\", issuer = \"https://a.example\")]\npub struct First;\n#[issues_tokens(audience = \"b\", issuer = \"https://b.example\")]\npub struct Second;\n"
            ),
            Err(TokenIssuanceCodegenError::AmbiguousTokenIssuance { first, second })
                if first == "crate::First" && second == "crate::Second"
        ));
    }

    #[test]
    fn rejects_a_declaration_without_an_audience() {
        assert!(matches!(
            rejection("issuer = \"https://issuer.example\""),
            TokenIssuanceCodegenError::MissingAudience { anchor } if anchor == "crate::Issuer"
        ));
    }

    #[test]
    fn rejects_an_audience_that_is_not_a_string() {
        assert!(matches!(
            rejection("audience = 5, issuer = \"https://issuer.example\""),
            TokenIssuanceCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "audience"
        ));
    }

    #[test]
    fn rejects_an_empty_audience() {
        assert!(matches!(
            rejection("audience = \"\", issuer = \"https://issuer.example\""),
            TokenIssuanceCodegenError::MalformedAudience { anchor, .. } if anchor == "crate::Issuer"
        ));
    }

    #[test]
    fn rejects_a_declaration_without_an_issuer() {
        assert!(matches!(
            rejection("audience = \"session\""),
            TokenIssuanceCodegenError::MissingIssuer { anchor } if anchor == "crate::Issuer"
        ));
    }

    #[test]
    fn rejects_an_issuer_that_is_not_a_string() {
        assert!(matches!(
            rejection("audience = \"session\", issuer = issuer"),
            TokenIssuanceCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "issuer"
        ));
    }

    #[test]
    fn rejects_an_issuer_over_plain_http() {
        assert!(matches!(
            rejection("audience = \"session\", issuer = \"http://issuer.example\""),
            TokenIssuanceCodegenError::MalformedIssuer { anchor, .. } if anchor == "crate::Issuer"
        ));
    }

    #[test]
    fn rejects_an_unrecognized_argument() {
        assert!(matches!(
            rejection("audience = \"session\", issuer = \"https://issuer.example\", lifetime = 5"),
            TokenIssuanceCodegenError::AttributeArguments(
                AttributeArgumentsError::UnrecognizedArgument { argument, .. }
            ) if argument == "lifetime"
        ));
    }

    #[test]
    fn rejects_arguments_that_do_not_parse() {
        assert!(matches!(
            rejection("= 5"),
            TokenIssuanceCodegenError::Index(AttributeError::Arguments(
                AttributeArgumentsError::Malformed { attribute_path, .. }
            )) if attribute_path == "issues_tokens"
        ));
    }

    #[test]
    fn rejects_a_declaration_that_is_not_anchored_by_a_struct() {
        assert!(matches!(
            read(
                "#[issues_tokens(audience = \"session\", issuer = \"https://issuer.example\")]\npub enum Issuer { Only }\n"
            ),
            Err(TokenIssuanceCodegenError::Anchor(DeclarationAnchorError::NotAStruct { path, .. }))
                if path == "crate::Issuer"
        ));
    }
}
