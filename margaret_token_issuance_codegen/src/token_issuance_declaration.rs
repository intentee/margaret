use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::matched_attribute::MatchedAttribute;
use margaret_registered_claims::audience::Audience;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

use crate::token_issuance_codegen_error::TokenIssuanceCodegenError;

#[derive(Debug, Eq, PartialEq)]
pub struct TokenIssuanceDeclaration {
    pub anchor: CanonicalPath,
    pub audience: Audience,
    pub issuer: IssuerIdentifier,
}

impl TokenIssuanceDeclaration {
    pub(crate) fn read(
        matched: &MatchedAttribute,
        anchor: &CanonicalPath,
    ) -> Result<Self, TokenIssuanceCodegenError> {
        matched.args()?.interpret(|reader| {
            let audience = reader
                .take_string("audience")?
                .ok_or_else(|| TokenIssuanceCodegenError::MissingAudience {
                    anchor: anchor.to_string(),
                })?
                .parse::<Audience>()
                .map_err(|source| TokenIssuanceCodegenError::MalformedAudience {
                    anchor: anchor.to_string(),
                    source,
                })?;
            let issuer = reader
                .take_string("issuer")?
                .ok_or_else(|| TokenIssuanceCodegenError::MissingIssuer {
                    anchor: anchor.to_string(),
                })?
                .parse::<IssuerIdentifier>()
                .map_err(|source| TokenIssuanceCodegenError::MalformedIssuer {
                    anchor: anchor.to_string(),
                    source,
                })?;

            Ok(Self {
                anchor: anchor.clone(),
                audience,
                issuer,
            })
        })
    }
}
