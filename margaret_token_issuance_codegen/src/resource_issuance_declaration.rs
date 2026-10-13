use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::matched_attribute::MatchedAttribute;
use margaret_attributes::tag::Tag;
use margaret_registered_claims::audience::Audience;
use margaret_registered_claims::audience_parsing::AudienceParsing;

use crate::token_issuance_codegen_error::TokenIssuanceCodegenError;

pub struct ResourceIssuanceDeclaration<'index> {
    pub anchor: &'index IndexedItem,
    pub audience: Audience,
    pub tag: Tag,
}

impl<'index> ResourceIssuanceDeclaration<'index> {
    pub(crate) fn read(
        matched: &MatchedAttribute,
        anchor: &'index IndexedItem,
    ) -> Result<Self, TokenIssuanceCodegenError> {
        let path = anchor.canonical_path();

        matched.args()?.interpret(|reader| {
            let tag = reader
                .take_positional_path()
                .ok_or_else(|| TokenIssuanceCodegenError::MissingResourceTag {
                    anchor: path.to_string(),
                })
                .and_then(|tag| {
                    Tag::from_path(&tag).ok_or_else(|| {
                        TokenIssuanceCodegenError::MalformedResourceTag {
                            anchor: path.to_string(),
                        }
                    })
                })?;
            let AudienceParsing::Accepted(audience) =
                Audience::parse(&reader.take_string("audience")?.ok_or_else(|| {
                    TokenIssuanceCodegenError::MissingResourceAudience {
                        anchor: path.to_string(),
                    }
                })?)
            else {
                return Err(TokenIssuanceCodegenError::EmptyResourceAudience {
                    anchor: path.to_string(),
                });
            };

            Ok(Self {
                anchor,
                audience,
                tag,
            })
        })
    }
}
