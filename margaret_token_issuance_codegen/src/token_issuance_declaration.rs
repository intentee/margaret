use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::matched_attribute::MatchedAttribute;
use margaret_attributes::tag::Tag;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

use crate::token_issuance_codegen_error::TokenIssuanceCodegenError;

pub struct TokenIssuanceDeclaration<'index> {
    pub anchor: &'index IndexedItem,
    pub issuer: IssuerIdentifier,
    pub tag: Tag,
}

impl<'index> TokenIssuanceDeclaration<'index> {
    pub(crate) fn read(
        matched: &MatchedAttribute,
        anchor: &'index IndexedItem,
    ) -> Result<Self, TokenIssuanceCodegenError> {
        let path = anchor.canonical_path();

        matched.args()?.interpret(|reader| {
            let tag = reader
                .take_positional_path()
                .ok_or_else(|| TokenIssuanceCodegenError::MissingTag {
                    anchor: path.to_string(),
                })
                .and_then(|tag| {
                    Tag::from_path(&tag).ok_or_else(|| TokenIssuanceCodegenError::MalformedTag {
                        anchor: path.to_string(),
                    })
                })?;
            let issuer = reader
                .take_string("issuer")?
                .ok_or_else(|| TokenIssuanceCodegenError::MissingIssuer {
                    anchor: path.to_string(),
                })?
                .parse::<IssuerIdentifier>()
                .map_err(|source| TokenIssuanceCodegenError::MalformedIssuer {
                    anchor: path.to_string(),
                    source,
                })?;

            Ok(Self {
                anchor,
                issuer,
                tag,
            })
        })
    }
}
