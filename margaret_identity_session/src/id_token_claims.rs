use serde_json::Value;
use uuid::Uuid;

use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims::RegisteredClaims;

pub struct IdTokenClaims {
    pub auth_time: NumericDate,
    pub client_id: &'static str,
    pub nonce: Option<String>,
    pub subject: Uuid,
}

impl IdTokenClaims {
    #[must_use]
    pub fn to_payload(&self, registered: &RegisteredClaims) -> Value {
        let mut payload = registered.to_json();

        payload.insert(
            "auth_time".to_string(),
            Value::Number(self.auth_time.seconds_since_epoch().into()),
        );
        payload.insert("azp".to_string(), Value::String(self.client_id.to_string()));

        if let Some(nonce) = &self.nonce {
            payload.insert("nonce".to_string(), Value::String(nonce.clone()));
        }

        payload.insert("sub".to_string(), Value::String(self.subject.to_string()));

        Value::Object(payload)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use uuid::Uuid;

    use margaret_registered_claims::audience_claim::AudienceClaim;
    use margaret_registered_claims::numeric_date::NumericDate;
    use margaret_registered_claims::registered_claims::RegisteredClaims;

    use super::IdTokenClaims;
    use crate::id_token_members::ID_TOKEN_MEMBERS;

    #[test]
    fn issues_exactly_the_members_it_advertises() {
        let payload = IdTokenClaims {
            auth_time: NumericDate::new(100),
            client_id: "portal",
            nonce: Some("nonce".to_string()),
            subject: Uuid::nil(),
        }
        .to_payload(&RegisteredClaims {
            aud: AudienceClaim::Single("portal".to_string()),
            exp: NumericDate::new(200),
            iat: Some(NumericDate::new(100)),
            iss: "https://issuer.localhost".to_string(),
            jti: None,
            nbf: None,
        });

        assert_eq!(
            payload
                .as_object()
                .expect("an id token payload is an object")
                .keys()
                .map(String::as_str)
                .collect::<BTreeSet<&str>>(),
            BTreeSet::from(ID_TOKEN_MEMBERS)
        );
    }
}
