use std::collections::BTreeMap;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::tag::Tag;
use margaret_declaration_anchor::declaration_anchor::declaration_anchor;

use crate::declared_token_issuance::DeclaredTokenIssuance;
use crate::resource_issuance_declaration::ResourceIssuanceDeclaration;
use crate::token_issuance_codegen_error::TokenIssuanceCodegenError;

pub struct DeclaredResourceIssuances<'index> {
    resources: BTreeMap<String, ResourceIssuanceDeclaration<'index>>,
}

impl<'index> DeclaredResourceIssuances<'index> {
    /// # Errors
    ///
    /// Returns `TokenIssuanceCodegenError` when a declaration is malformed, when resource tokens
    /// are declared without the token issuance that signs them, or when an audience names the
    /// issuer or another resource.
    pub fn read(
        index: &'index AttributeIndex,
        issuance: &DeclaredTokenIssuance,
    ) -> Result<Self, TokenIssuanceCodegenError> {
        let mut resources: BTreeMap<String, ResourceIssuanceDeclaration<'index>> = BTreeMap::new();

        for matched in index.select_framework_attribute(FrameworkAttribute::IssuesResourceTokens) {
            let anchor =
                declaration_anchor(index, &matched, FrameworkAttribute::IssuesResourceTokens)?;
            let declared = ResourceIssuanceDeclaration::read(&matched, anchor.item)?;

            match issuance {
                DeclaredTokenIssuance::Absent => {
                    return Err(TokenIssuanceCodegenError::ResourceWithoutTokenIssuance {
                        anchor: declared.anchor.canonical_path().to_string(),
                    });
                }
                DeclaredTokenIssuance::Declared(issuance)
                    if issuance.issuer.as_str() == declared.audience.as_str() =>
                {
                    return Err(TokenIssuanceCodegenError::ResourceAudienceNamesIssuer {
                        anchor: declared.anchor.canonical_path().to_string(),
                        audience: declared.audience.to_string(),
                    });
                }
                DeclaredTokenIssuance::Declared(_) => {}
            }

            if let Some(first) = resources.get(declared.audience.as_str()) {
                return Err(TokenIssuanceCodegenError::DuplicateResourceAudience {
                    audience: declared.audience.to_string(),
                    first: first.anchor.canonical_path().to_string(),
                    second: declared.anchor.canonical_path().to_string(),
                });
            }

            resources.insert(declared.audience.to_string(), declared);
        }

        Ok(Self { resources })
    }

    #[must_use]
    pub fn find(&self, tag: &Tag) -> Option<&ResourceIssuanceDeclaration<'index>> {
        self.resources
            .values()
            .find(|resource| resource.tag == *tag)
    }

    pub fn resources(&self) -> impl Iterator<Item = &ResourceIssuanceDeclaration<'index>> {
        self.resources.values()
    }
}

#[cfg(test)]
mod tests {
    use quote::format_ident;

    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_attributes::attribute_error::AttributeError;
    use margaret_attributes::tag::Tag;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_declaration_anchor::declaration_anchor_error::DeclarationAnchorError;

    use super::DeclaredResourceIssuances;
    use crate::declared_token_issuance::DeclaredTokenIssuance;
    use crate::token_issuance_codegen_error::TokenIssuanceCodegenError;

    const ISSUANCE: &str =
        "#[issues_tokens(provider, issuer = \"https://issuer.example\")]\npub struct Issuer;\n";

    fn read<TOutcome>(
        source: &str,
        outcome: impl FnOnce(Result<DeclaredResourceIssuances, TokenIssuanceCodegenError>) -> TOutcome,
    ) -> TOutcome {
        let indexed = IndexedSource::new(source);
        let issuance =
            DeclaredTokenIssuance::read(&indexed.index).expect("the token issuance is read");

        outcome(DeclaredResourceIssuances::read(&indexed.index, &issuance))
    }

    fn rejection(resources: &str) -> TokenIssuanceCodegenError {
        read(&format!("{ISSUANCE}{resources}"), |read| {
            read.err().expect("the resources are rejected")
        })
    }

    #[test]
    fn finds_a_resource_by_its_tag() {
        read(
            &format!(
                "{ISSUANCE}#[issues_resource_tokens(attachments, audience = \"attachments\")]\npub struct Attachments;\n"
            ),
            |read| {
                let resources = read.expect("the resources are read");

                assert!(matches!(
                    resources.find(&Tag::from_ident(format_ident!("attachments"))),
                    Some(resource) if resource.audience.as_str() == "attachments"
                        && resource.anchor.canonical_path().to_string() == "crate::Attachments"
                ));
                assert!(
                    resources
                        .find(&Tag::from_ident(format_ident!("reports")))
                        .is_none()
                );
                assert_eq!(resources.resources().count(), 1);
            },
        );
    }

    #[test]
    fn rejects_a_resource_without_a_tag() {
        assert!(matches!(
            rejection("#[issues_resource_tokens(audience = \"attachments\")]\npub struct Attachments;\n"),
            TokenIssuanceCodegenError::MissingResourceTag { anchor } if anchor == "crate::Attachments"
        ));
    }

    #[test]
    fn rejects_a_tag_that_is_not_a_plain_name() {
        assert!(matches!(
            rejection("#[issues_resource_tokens(files::attachments, audience = \"attachments\")]\npub struct Attachments;\n"),
            TokenIssuanceCodegenError::MalformedResourceTag { anchor } if anchor == "crate::Attachments"
        ));
    }

    #[test]
    fn rejects_a_resource_without_an_audience() {
        assert!(matches!(
            rejection("#[issues_resource_tokens(attachments)]\npub struct Attachments;\n"),
            TokenIssuanceCodegenError::MissingResourceAudience { anchor } if anchor == "crate::Attachments"
        ));
    }

    #[test]
    fn rejects_an_empty_audience() {
        assert!(matches!(
            rejection("#[issues_resource_tokens(attachments, audience = \"\")]\npub struct Attachments;\n"),
            TokenIssuanceCodegenError::EmptyResourceAudience { anchor } if anchor == "crate::Attachments"
        ));
    }

    #[test]
    fn rejects_an_audience_that_is_not_a_string() {
        assert!(matches!(
            rejection("#[issues_resource_tokens(attachments, audience = 5)]\npub struct Attachments;\n"),
            TokenIssuanceCodegenError::AttributeArguments(
                AttributeArgumentsError::UnexpectedArgument { key, .. }
            ) if key == "audience"
        ));
    }

    #[test]
    fn rejects_arguments_that_do_not_parse() {
        assert!(matches!(
            rejection("#[issues_resource_tokens(= 5)]\npub struct Attachments;\n"),
            TokenIssuanceCodegenError::Index(AttributeError::Arguments(
                AttributeArgumentsError::Malformed { attribute_path, .. }
            )) if attribute_path == "issues_resource_tokens"
        ));
    }

    #[test]
    fn rejects_a_resource_that_is_not_anchored_by_a_struct() {
        assert!(matches!(
            rejection("#[issues_resource_tokens(attachments, audience = \"attachments\")]\npub enum Attachments { Only }\n"),
            TokenIssuanceCodegenError::Anchor(DeclarationAnchorError::NotAStruct { path, .. })
                if path == "crate::Attachments"
        ));
    }

    #[test]
    fn rejects_resource_tokens_without_the_token_issuance() {
        read(
            "#[issues_resource_tokens(attachments, audience = \"attachments\")]\npub struct Attachments;\n",
            |read| {
                assert!(matches!(
                    read,
                    Err(TokenIssuanceCodegenError::ResourceWithoutTokenIssuance { anchor })
                        if anchor == "crate::Attachments"
                ));
            },
        );
    }

    #[test]
    fn rejects_a_resource_addressed_like_the_issuer() {
        assert_eq!(
            rejection("#[issues_resource_tokens(attachments, audience = \"https://issuer.example\")]\npub struct Attachments;\n")
                .to_string(),
            "#[issues_resource_tokens] on 'crate::Attachments' declares the audience 'https://issuer.example', which names the issuer, so its tokens would pass where the issuer is addressed"
        );
    }

    #[test]
    fn rejects_two_resources_of_one_audience() {
        assert!(matches!(
            rejection("#[issues_resource_tokens(attachments, audience = \"files\")]\npub struct Attachments;\n#[issues_resource_tokens(uploads, audience = \"files\")]\npub struct Uploads;\n"),
            TokenIssuanceCodegenError::DuplicateResourceAudience { audience, first, second }
                if audience == "files" && first == "crate::Attachments" && second == "crate::Uploads"
        ));
    }
}
