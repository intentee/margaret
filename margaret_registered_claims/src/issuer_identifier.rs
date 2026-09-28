use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;

use url::Url;

use crate::registered_claims_error::RegisteredClaimsError;

const ISSUER_SCHEME: &str = "https";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssuerIdentifier {
    original: String,
}

impl IssuerIdentifier {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.original
    }
}

impl Display for IssuerIdentifier {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.original)
    }
}

impl FromStr for IssuerIdentifier {
    type Err = RegisteredClaimsError;

    fn from_str(original: &str) -> Result<Self, Self::Err> {
        let url =
            Url::parse(original).map_err(|source| RegisteredClaimsError::IssuerMalformed {
                original: original.to_string(),
                source,
            })?;

        if url.scheme() != ISSUER_SCHEME {
            return Err(RegisteredClaimsError::IssuerNotHttps {
                original: original.to_string(),
                scheme: url.scheme().to_string(),
            });
        }

        if url.query().is_some() {
            return Err(RegisteredClaimsError::IssuerHasQuery {
                original: original.to_string(),
            });
        }

        if url.fragment().is_some() {
            return Err(RegisteredClaimsError::IssuerHasFragment {
                original: original.to_string(),
            });
        }

        Ok(Self {
            original: original.to_string(),
        })
    }
}
