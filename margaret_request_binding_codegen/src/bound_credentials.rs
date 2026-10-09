use std::collections::HashSet;

use margaret_attributes::tag::Tag;

use crate::request_binding_error::RequestBindingError;

pub(crate) enum BoundCredentials {
    Introspected,
    Session,
    Unbound,
    VerifiedJwts(HashSet<Tag>),
}

impl BoundCredentials {
    pub(crate) fn bind_introspected(&mut self, subject: &str) -> Result<(), RequestBindingError> {
        match self {
            Self::Introspected => Err(RequestBindingError::DuplicateIntrospectedBearerToken {
                subject: subject.to_string(),
            }),
            Self::Unbound => {
                *self = Self::Introspected;

                Ok(())
            }
            Self::Session => Err(RequestBindingError::SessionMixedWithBearerToken {
                subject: subject.to_string(),
            }),
            Self::VerifiedJwts(_) => Err(RequestBindingError::MixedBearerTokenCarriers {
                subject: subject.to_string(),
            }),
        }
    }

    pub(crate) fn bind_session(&mut self, subject: &str) -> Result<(), RequestBindingError> {
        match self {
            Self::Introspected | Self::VerifiedJwts(_) => {
                Err(RequestBindingError::SessionMixedWithBearerToken {
                    subject: subject.to_string(),
                })
            }
            Self::Session => Err(RequestBindingError::DuplicateSession {
                subject: subject.to_string(),
            }),
            Self::Unbound => {
                *self = Self::Session;

                Ok(())
            }
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
            Self::Session => Err(RequestBindingError::SessionMixedWithBearerToken {
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
