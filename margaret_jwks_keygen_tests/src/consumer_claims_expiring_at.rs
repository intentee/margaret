use crate::consumer_audience::CONSUMER_AUDIENCE;
use crate::consumer_claims::ConsumerClaims;
use crate::consumer_grant::ConsumerGrant;
use crate::consumer_issuer::CONSUMER_ISSUER;

#[must_use]
pub fn consumer_claims_expiring_at(exp: usize) -> ConsumerClaims {
    ConsumerClaims {
        aud: vec![CONSUMER_AUDIENCE.to_string()],
        env: "production".to_string(),
        exp,
        groups: Some(vec!["editors".to_string(), "readers".to_string()]),
        iat: 1,
        idp: "corporate-directory".to_string(),
        is_service_account: Some(false),
        iss: CONSUMER_ISSUER.to_string(),
        name: "Ada Lovelace".to_string(),
        preferred_username: "ada".to_string(),
        resources: Some(vec![
            ConsumerGrant {
                permission: vec!["read".to_string(), "write".to_string()],
                resource_id: "repository-0194b726b34e72b0b45550b88a967076".to_string(),
            },
            ConsumerGrant {
                permission: vec!["admin".to_string()],
                resource_id: "repository-*".to_string(),
            },
        ]),
        sub: "3f2a1c04-not-a-uuid-shaped-subject".to_string(),
    }
}
