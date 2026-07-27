use async_trait::async_trait;

use crate::access_decision::AccessDecision;
use crate::access_policy::AccessPolicy;
use crate::request::Request;

pub struct PublicAccess;

#[async_trait]
impl AccessPolicy for PublicAccess {
    async fn decide(&self, _request: &Request) -> anyhow::Result<AccessDecision> {
        Ok(AccessDecision::Allowed)
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use super::PublicAccess;
    use crate::access_decision::AccessDecision;
    use crate::access_policy::AccessPolicy;
    use crate::request::Request;

    #[tokio::test]
    async fn permits_explicitly_public_requests() {
        let decision = PublicAccess
            .decide(&Request::new(Method::GET, "/public".to_string()))
            .await
            .expect("public access cannot have a system failure");

        assert_eq!(
            std::mem::discriminant(&decision),
            std::mem::discriminant(&AccessDecision::Allowed)
        );
    }
}
