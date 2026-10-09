use std::str::FromStr;

use url::Host;

use crate::sessions_error::SessionsError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CookieDomain {
    domain: String,
}

impl CookieDomain {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.domain
    }
}

impl FromStr for CookieDomain {
    type Err = SessionsError;

    fn from_str(domain: &str) -> Result<Self, Self::Err> {
        match Host::parse(domain) {
            Ok(Host::Domain(parsed)) => Ok(Self { domain: parsed }),
            Ok(Host::Ipv4(_) | Host::Ipv6(_)) => Err(SessionsError::CookieDomainNotADomain {
                domain: domain.to_string(),
            }),
            Err(source) => Err(SessionsError::MalformedCookieDomain {
                domain: domain.to_string(),
                source,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CookieDomain;
    use crate::sessions_error::SessionsError;

    #[test]
    fn reads_a_domain_name() {
        assert_eq!(
            "Intentee.AI"
                .parse::<CookieDomain>()
                .expect("the domain is read")
                .as_str(),
            "intentee.ai"
        );
    }

    #[test]
    fn rejects_an_address_in_place_of_a_domain() {
        assert!(matches!(
            "127.0.0.1".parse::<CookieDomain>(),
            Err(SessionsError::CookieDomainNotADomain { domain }) if domain == "127.0.0.1"
        ));
    }

    #[test]
    fn rejects_a_malformed_domain() {
        assert!(matches!(
            "intentee ai".parse::<CookieDomain>(),
            Err(SessionsError::MalformedCookieDomain { domain, .. }) if domain == "intentee ai"
        ));
    }
}
