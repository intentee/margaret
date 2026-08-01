use async_trait::async_trait;
use url::Url;

use margaret_jwks_endpoint::provides_endpoint::ProvidesEndpoint;
use margaret_jwt_claims::expected_claims::ExpectedClaims;
use margaret_jwt_claims::provides_expected_claims::ProvidesExpectedClaims;

pub struct StaticIssuer {
    endpoint: Url,
    expected_claims: ExpectedClaims,
}

impl StaticIssuer {
    #[must_use]
    pub fn new(endpoint: Url, expected_claims: ExpectedClaims) -> Self {
        Self {
            endpoint,
            expected_claims,
        }
    }
}

#[async_trait]
impl ProvidesEndpoint for StaticIssuer {
    async fn provide(&self) -> anyhow::Result<Url> {
        Ok(self.endpoint.clone())
    }
}

impl ProvidesExpectedClaims for StaticIssuer {
    fn expected_claims(&self) -> ExpectedClaims {
        self.expected_claims.clone()
    }
}

#[cfg(test)]
mod tests {
    use url::Url;

    use margaret_jwks_endpoint::provides_endpoint::ProvidesEndpoint as _;
    use margaret_jwt_claims::expected_claims::ExpectedClaims;
    use margaret_jwt_claims::provides_expected_claims::ProvidesExpectedClaims as _;

    use super::StaticIssuer;

    fn expected_claims() -> ExpectedClaims {
        ExpectedClaims {
            audience: "dashboard".to_string(),
            issuer: "https://issuer.example.org".to_string(),
        }
    }

    fn issuer() -> StaticIssuer {
        StaticIssuer::new(
            Url::parse("https://issuer.example.org/.well-known/jwks.json")
                .expect("the url parses"),
            expected_claims(),
        )
    }

    #[tokio::test]
    async fn provides_the_configured_url() {
        assert_eq!(
            issuer()
                .provide()
                .await
                .expect("the endpoint resolves")
                .as_str(),
            "https://issuer.example.org/.well-known/jwks.json"
        );
    }

    #[test]
    fn provides_the_configured_expected_claims() {
        assert_eq!(issuer().expected_claims(), expected_claims());
    }
}
