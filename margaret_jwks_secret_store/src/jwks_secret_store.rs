use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use serde::de::DeserializeOwned;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwks_keygen::signs_claims::SignsClaims;
use margaret_token_signer::access_token_minting::AccessTokenMinting;
use margaret_token_signer::mint_access_token::mint_access_token;

use crate::jwks_secret_store_error::JwksSecretStoreError;

pub struct JwksSecretStore {
    holder: JwksSecretHolder,
}

impl JwksSecretStore {
    #[must_use]
    pub fn new(holder: JwksSecretHolder) -> Self {
        Self { holder }
    }

    /// # Errors
    ///
    /// Returns `JwksSecretStoreError::SecretUnavailable` before the first roll.
    pub fn mint_access_token(
        &self,
        refresh_token: &str,
        now: DateTime<Utc>,
    ) -> Result<AccessTokenMinting, JwksSecretStoreError> {
        let secret = self.current_secret()?;

        Ok(mint_access_token(&secret, refresh_token, now))
    }

    /// # Errors
    ///
    /// Returns `JwksSecretStoreError::Sign`.
    pub async fn sign<TClaims: Send + Serialize + Sync>(
        &self,
        claims: &TClaims,
    ) -> Result<String, JwksSecretStoreError> {
        let secret = self.current_secret()?;

        secret
            .current()
            .sign(claims)
            .await
            .map_err(|source| JwksSecretStoreError::Sign { source })
    }

    /// # Errors
    ///
    /// Returns `JwksSecretStoreError::SecretUnavailable` before the first roll.
    pub fn verify<TClaims: DeserializeOwned>(
        &self,
        token: &str,
    ) -> Result<JwksSecretVerificationResult<TClaims>, JwksSecretStoreError> {
        Ok(self.current_secret()?.verify_any::<TClaims>(token))
    }

    fn current_secret(&self) -> Result<Arc<JwksSecret>, JwksSecretStoreError> {
        self.holder
            .get()
            .ok_or(JwksSecretStoreError::SecretUnavailable)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use serde::Serialize;
    use serde::Serializer;
    use serde::ser::Error;
    use uuid::Uuid;

    use margaret_identity_session::refresh_token_claims::RefreshTokenClaims;
    use margaret_jwks_keygen::jwks_secret::JwksSecret;
    use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
    use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
    use margaret_token_signer::access_token_minting::AccessTokenMinting;
    use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
    use margaret_token_signer_tests::refresh_claims::refresh_claims;
    use margaret_token_signer_tests::sign_refresh_token::sign_refresh_token;
    use margaret_token_signer_tests::unix_time::unix_time;

    use super::JwksSecretStore;

    struct Unserializable;

    impl Serialize for Unserializable {
        fn serialize<Target: Serializer>(
            &self,
            _serializer: Target,
        ) -> Result<Target::Ok, Target::Error> {
            Err(Target::Error::custom("this value cannot be serialized"))
        }
    }

    fn store_of(secret: JwksSecret) -> JwksSecretStore {
        let holder = JwksSecretHolder::default();

        holder.set(Some(Arc::new(secret)));

        JwksSecretStore::new(holder)
    }

    fn unrolled_store() -> JwksSecretStore {
        JwksSecretStore::new(JwksSecretHolder::default())
    }

    fn minted(minting: &AccessTokenMinting) -> bool {
        match minting {
            AccessTokenMinting::Minted(_) => true,
            AccessTokenMinting::ExpiredRefreshToken
            | AccessTokenMinting::MalformedRefreshTokenClaims(_)
            | AccessTokenMinting::RefreshTokenSignedWithNextKey
            | AccessTokenMinting::RejectedRefreshToken(_) => false,
        }
    }

    fn verified_jti(result: JwksSecretVerificationResult<RefreshTokenClaims>) -> Option<Uuid> {
        match result {
            JwksSecretVerificationResult::SignedWithCurrent(claims)
            | JwksSecretVerificationResult::SignedWithPrevious(claims) => Some(claims.jti),
            JwksSecretVerificationResult::MalformedClaims(_)
            | JwksSecretVerificationResult::Rejected(_)
            | JwksSecretVerificationResult::SignedWithNextKey => None,
        }
    }

    #[test]
    fn mints_an_access_token_from_a_valid_refresh_token() {
        let secret = fresh_p256_secret();
        let refresh_token = sign_refresh_token(secret.current(), &refresh_claims(1_000));

        assert!(minted(
            &store_of(secret)
                .mint_access_token(&refresh_token, unix_time(500))
                .expect("the signing secret is usable")
        ));
    }

    #[test]
    fn reports_a_malformed_refresh_token_as_a_minting_outcome() {
        assert!(!minted(
            &store_of(fresh_p256_secret())
                .mint_access_token("not.a.valid.token", unix_time(500))
                .expect("the signing secret is usable")
        ));
    }

    #[tokio::test]
    async fn signs_claims_with_the_current_key() {
        let secret = fresh_p256_secret();
        let claims = refresh_claims(1_000);
        let token = store_of(secret.clone())
            .sign(&claims.to_json())
            .await
            .expect("the claims are signed");

        assert_eq!(
            verified_jti(secret.verify_any::<RefreshTokenClaims>(&token)),
            Some(claims.jti)
        );
    }

    #[test]
    fn reports_a_malformed_token_as_a_verification_outcome() {
        assert_eq!(
            verified_jti(
                store_of(fresh_p256_secret())
                    .verify::<RefreshTokenClaims>("not.a.valid.token")
                    .expect("the signing secret is usable")
            ),
            None
        );
    }

    #[test]
    fn verifies_a_token_signed_with_the_retired_key() {
        let secret = fresh_p256_secret();
        let claims = refresh_claims(1_000);
        let token = sign_refresh_token(secret.current(), &claims);
        let rotated = secret.rotate().expect("the signing secret rotates");

        assert_eq!(
            verified_jti(
                store_of(rotated)
                    .verify::<RefreshTokenClaims>(&token)
                    .expect("the signing secret is usable")
            ),
            Some(claims.jti)
        );
    }

    #[test]
    fn reports_the_secret_is_unavailable_when_verifying_before_a_roll() {
        assert!(
            unrolled_store()
                .verify::<RefreshTokenClaims>("token")
                .is_err_and(|error| error.to_string().contains("not available yet"))
        );
    }

    #[test]
    fn reports_the_secret_is_unavailable_when_minting_before_a_roll() {
        assert!(
            unrolled_store()
                .mint_access_token("token", unix_time(0))
                .is_err_and(|error| error.to_string().contains("not available yet"))
        );
    }

    #[tokio::test]
    async fn reports_the_secret_is_unavailable_when_signing_before_a_roll() {
        assert!(
            unrolled_store()
                .sign(&refresh_claims(1_000).to_json())
                .await
                .is_err_and(|error| error.to_string().contains("not available yet"))
        );
    }

    #[tokio::test]
    async fn reports_a_sign_failure_for_unserializable_claims() {
        assert!(
            store_of(fresh_p256_secret())
                .sign(&Unserializable)
                .await
                .is_err_and(|error| error.to_string().contains("failed to sign claims"))
        );
    }
}
