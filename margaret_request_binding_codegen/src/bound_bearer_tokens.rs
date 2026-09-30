use std::collections::HashSet;

use margaret_attributes::tag::Tag;

use crate::request_binding_error::RequestBindingError;

pub(crate) enum BoundBearerTokens {
    Introspected,
    Unbound,
    VerifiedJwts(HashSet<Tag>),
}

impl BoundBearerTokens {
    pub(crate) fn bind_introspected(&mut self, subject: &str) -> Result<(), RequestBindingError> {
        match self {
            Self::Introspected => Err(RequestBindingError::DuplicateIntrospectedBearerToken {
                subject: subject.to_string(),
            }),
            Self::Unbound => {
                *self = Self::Introspected;

                Ok(())
            }
            Self::VerifiedJwts(_) => Err(RequestBindingError::MixedBearerTokenCarriers {
                subject: subject.to_string(),
            }),
        }
    }

    pub(crate) fn bind_verified_jwt(
        &mut self,
        issuer: &Tag,
        subject: &str,
    ) -> Result<(), RequestBindingError> {
        match self {
            Self::Introspected => Err(RequestBindingError::MixedBearerTokenCarriers {
                subject: subject.to_string(),
            }),
            Self::Unbound => {
                *self = Self::VerifiedJwts(HashSet::from([issuer.clone()]));

                Ok(())
            }
            Self::VerifiedJwts(issuers) => {
                if issuers.insert(issuer.clone()) {
                    Ok(())
                } else {
                    Err(RequestBindingError::DuplicateBearerTokenIssuer {
                        subject: subject.to_string(),
                        issuer: issuer.to_string(),
                    })
                }
            }
        }
    }
}
