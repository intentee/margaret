use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_declaration_anchor::declaration_anchor::declaration_anchor;

use crate::token_issuance_codegen_error::TokenIssuanceCodegenError;
use crate::token_issuance_declaration::TokenIssuanceDeclaration;

pub enum DeclaredTokenIssuance<'index> {
    Absent,
    Declared(TokenIssuanceDeclaration<'index>),
}

impl<'index> DeclaredTokenIssuance<'index> {
    /// # Errors
    ///
    /// Returns `TokenIssuanceCodegenError` when a declaration is malformed or when more than one
    /// struct declares the token issuance.
    pub fn read(index: &'index AttributeIndex) -> Result<Self, TokenIssuanceCodegenError> {
        let mut declarations = Vec::new();

        for matched in index.select_framework_attribute(FrameworkAttribute::IssuesTokens) {
            let anchor = declaration_anchor(index, &matched, FrameworkAttribute::IssuesTokens)?;

            declarations.push(TokenIssuanceDeclaration::read(&matched, anchor.item)?);
        }

        let mut declarations = declarations.into_iter();
        let Some(declared) = declarations.next() else {
            return Ok(Self::Absent);
        };

        match declarations.next() {
            Some(another) => Err(TokenIssuanceCodegenError::AmbiguousTokenIssuance {
                first: declared.anchor.canonical_path().to_string(),
                second: another.anchor.canonical_path().to_string(),
            }),
            None => Ok(Self::Declared(declared)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem::discriminant;

    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_attributes::attribute_error::AttributeError;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_declaration_anchor::declaration_anchor_error::DeclarationAnchorError;

    use super::DeclaredTokenIssuance;
    use crate::token_issuance_codegen_error::TokenIssuanceCodegenError;
    use crate::token_issuance_declaration::TokenIssuanceDeclaration;

    fn read<TOutcome>(
        source: &str,
        outcome: impl FnOnce(Result<DeclaredTokenIssuance, TokenIssuanceCodegenError>) -> TOutcome,
    ) -> TOutcome {
        outcome(DeclaredTokenIssuance::read(
            &IndexedSource::new(source).index,
        ))
    }

    fn rejection(arguments: &str) -> TokenIssuanceCodegenError {
        read(
            &format!("#[issues_tokens({arguments})]\npub struct Issuer;\n"),
            |read| read.err().expect("the declaration is rejected"),
        )
    }

    #[test]
    fn reads_the_declared_tag_audience_and_issuer() {
        read(
            "#[issues_tokens(provider, audience = \"session\", issuer = \"https://issuer.example\")]\npub struct Issuer;\n",
            |read| {
                assert!(matches!(
                    read,
                    Ok(DeclaredTokenIssuance::Declared(TokenIssuanceDeclaration {
                        anchor,
                        audience,
                        issuer,
                        tag,
                    })) if anchor.canonical_path().to_string() == "crate::Issuer"
                        && audience.as_str() == "session"
                        && issuer.as_str() == "https://issuer.example"
                        && tag.to_string() == "provider"
                ));
            },
        );
    }

    #[test]
    fn reads_no_issuance_when_nothing_declares_it() {
        read("pub struct Issuer;\n", |read| {
            assert_eq!(
                discriminant(&read.expect("the absence is read")),
                discriminant(&DeclaredTokenIssuance::Absent)
            );
        });
    }

    #[test]
    fn rejects_a_declaration_without_a_tag() {
        assert!(matches!(
            rejection("audience = \"session\", issuer = \"https://issuer.example\""),
            TokenIssuanceCodegenError::MissingTag { anchor } if anchor == "crate::Issuer"
        ));
    }

    #[test]
    fn rejects_a_tag_that_is_not_a_plain_name() {
        assert!(matches!(
            rejection("own::provider, audience = \"session\", issuer = \"https://issuer.example\""),
            TokenIssuanceCodegenError::MalformedTag { anchor } if anchor == "crate::Issuer"
        ));
    }

    #[test]
    fn rejects_two_declarations_of_the_token_issuance() {
        read(
            "#[issues_tokens(provider, audience = \"a\", issuer = \"https://a.example\")]\npub struct First;\n#[issues_tokens(provider, audience = \"b\", issuer = \"https://b.example\")]\npub struct Second;\n",
            |read| {
                assert!(matches!(
                    read,
                    Err(TokenIssuanceCodegenError::AmbiguousTokenIssuance { first, second })
                        if first == "crate::First" && second == "crate::Second"
                ));
            },
        );
    }

    #[test]
    fn rejects_a_declaration_without_an_audience() {
        assert!(matches!(
            rejection("provider, issuer = \"https://issuer.example\""),
            TokenIssuanceCodegenError::MissingAudience { anchor } if anchor == "crate::Issuer"
        ));
    }

    #[test]
    fn rejects_an_audience_that_is_not_a_string() {
        assert!(matches!(
            rejection("provider, audience = 5, issuer = \"https://issuer.example\""),
            TokenIssuanceCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "audience"
        ));
    }

    #[test]
    fn rejects_an_empty_audience() {
        assert!(matches!(
            rejection("provider, audience = \"\", issuer = \"https://issuer.example\""),
            TokenIssuanceCodegenError::EmptyAudience { anchor } if anchor == "crate::Issuer"
        ));
    }

    #[test]
    fn rejects_a_declaration_without_an_issuer() {
        assert!(matches!(
            rejection("provider, audience = \"session\""),
            TokenIssuanceCodegenError::MissingIssuer { anchor } if anchor == "crate::Issuer"
        ));
    }

    #[test]
    fn rejects_an_issuer_that_is_not_a_string() {
        assert!(matches!(
            rejection("provider, audience = \"session\", issuer = issuer"),
            TokenIssuanceCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "issuer"
        ));
    }

    #[test]
    fn rejects_an_issuer_over_plain_http() {
        assert!(matches!(
            rejection("provider, audience = \"session\", issuer = \"http://issuer.example\""),
            TokenIssuanceCodegenError::MalformedIssuer { anchor, .. } if anchor == "crate::Issuer"
        ));
    }

    #[test]
    fn rejects_an_unrecognized_argument() {
        assert!(matches!(
            rejection("provider, audience = \"session\", issuer = \"https://issuer.example\", lifetime = 5"),
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
        read(
            "#[issues_tokens(provider, audience = \"session\", issuer = \"https://issuer.example\")]\npub enum Issuer { Only }\n",
            |read| {
                assert!(matches!(
                    read,
                    Err(TokenIssuanceCodegenError::Anchor(DeclarationAnchorError::NotAStruct { path, .. }))
                        if path == "crate::Issuer"
                ));
            },
        );
    }
}
