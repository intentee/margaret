use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use serde::de::DeserializeOwned;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwks_keygen::signs_claims::SignsClaims;
use margaret_jwks_keygen::verifies_any_token::VerifiesAnyToken;
use margaret_token_signer::mint_access_token::mint_access_token;
use margaret_token_signer::mint_access_token_outcome::MintAccessTokenOutcome;

use crate::jwks_secret_store_error::JwksSecretStoreError;

pub struct JwksSecretStore {
    holder: JwksSecretHolder,
}

impl JwksSecretStore {
    #[must_use]
    pub fn new(holder: JwksSecretHolder) -> Self {
        Self { holder }
    }

    pub async fn mint_access_token(
        &self,
        refresh_token: &str,
        now: DateTime<Utc>,
    ) -> Result<MintAccessTokenOutcome, JwksSecretStoreError> {
        let secret = self.current_secret()?;

        mint_access_token(&secret, refresh_token, now)
            .await
            .map_err(|source| JwksSecretStoreError::Mint { source })
    }

    pub async fn sign<TClaims: Send + Serialize + Sync>(
        &self,
        claims: &TClaims,
    ) -> Result<String, JwksSecretStoreError> {
        let secret = self.current_secret()?;

        secret
            .current
            .signing
            .sign(claims)
            .await
            .map_err(|source| JwksSecretStoreError::Sign { source })
    }

    pub fn verify<TClaims: DeserializeOwned>(
        &self,
        token: &str,
    ) -> Result<JwksSecretVerificationResult<TClaims>, JwksSecretStoreError> {
        let secret = self.current_secret()?;

        secret
            .verify_any::<TClaims>(token)
            .map_err(|source| JwksSecretStoreError::Verify { source })
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

    use margaret_identity_session::refresh_token_claims::RefreshTokenClaims;
    use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
    use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
    use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
    use margaret_jwks_keygen::token_verification::TokenVerification;
    use margaret_jwks_keygen::verifies_token::VerifiesToken;
    use margaret_token_signer::mint_access_token_outcome::MintAccessTokenOutcome;
    use margaret_token_signer::minted_tokens::MintedTokens;
    use margaret_token_signer::refresh_token_rejection::RefreshTokenRejection;
    use margaret_token_signer::token_signer_error::TokenSignerError;
    use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
    use margaret_token_signer_tests::refresh_claims::refresh_claims;
    use margaret_token_signer_tests::sign_refresh_token::sign_refresh_token;
    use margaret_token_signer_tests::unix_time::unix_time;

    use super::JwksSecretStore;
    use crate::jwks_secret_store_error::JwksSecretStoreError;

    struct Unserializable;

    impl Serialize for Unserializable {
        fn serialize<Target: Serializer>(
            &self,
            _serializer: Target,
        ) -> Result<Target::Ok, Target::Error> {
            Err(Target::Error::custom("this value cannot be serialized"))
        }
    }

    fn store_with_secret() -> (
        JwksSecretStore,
        margaret_jwks_keygen::jwks_secret::JwksSecret,
    ) {
        let secret = fresh_p256_secret();
        let holder = JwksSecretHolder::default();

        holder.set(Some(Arc::new(secret.clone())));

        (JwksSecretStore::new(holder), secret)
    }

    #[tokio::test]
    async fn mints_an_access_token_from_a_valid_refresh_token() {
        let (store, secret) = store_with_secret();
        let refresh_token =
            sign_refresh_token(&secret.current.signing, &refresh_claims(1_000)).await;

        let outcome = store
            .mint_access_token(&refresh_token, unix_time(500))
            .await
            .expect("the access token is minted");

        assert_eq!(
            std::mem::discriminant(&outcome),
            std::mem::discriminant(&MintAccessTokenOutcome::Minted(MintedTokens {
                access_token: "comparison".to_string(),
                refresh_token: "comparison".to_string(),
            }))
        );
    }

    #[tokio::test]
    async fn signs_claims_with_the_current_key() {
        let (store, secret) = store_with_secret();
        let claims = refresh_claims(1_000);

        let token = store.sign(&claims).await.expect("the claims are signed");
        let verification = secret
            .current
            .public
            .verify::<RefreshTokenClaims>(&token)
            .expect("the token verifies");

        assert!(matches!(
            verification,
            TokenVerification::Verified(verified) if verified.sub == claims.sub
        ));
    }

    #[tokio::test]
    async fn verifies_a_token_signed_with_the_current_key() {
        let (store, _secret) = store_with_secret();
        let claims = refresh_claims(1_000);
        let token = store.sign(&claims).await.expect("the claims are signed");

        let verification = store
            .verify::<RefreshTokenClaims>(&token)
            .expect("the token verifies");

        assert!(matches!(
            verification,
            JwksSecretVerificationResult::SignedWithCurrent(verified) if verified.sub == claims.sub
        ));
    }

    #[test]
    fn reports_the_secret_is_unavailable_when_verifying_before_a_roll() {
        let store = JwksSecretStore::new(JwksSecretHolder::default());

        assert!(
            store
                .verify::<RefreshTokenClaims>("token")
                .is_err_and(|error| error.to_string().contains("not available yet"))
        );
    }

    #[test]
    fn reports_a_verify_failure_for_a_malformed_token() {
        let (store, _secret) = store_with_secret();

        assert!(
            store
                .verify::<RefreshTokenClaims>("not.a.valid.token")
                .is_err_and(|error| error.to_string().contains("failed to verify a token"))
        );
    }

    #[tokio::test]
    async fn reports_the_secret_is_unavailable_when_minting_before_a_roll() {
        let store = JwksSecretStore::new(JwksSecretHolder::default());

        assert!(
            store
                .mint_access_token("token", unix_time(0))
                .await
                .is_err_and(|error| error.to_string().contains("not available yet"))
        );
    }

    #[tokio::test]
    async fn reports_the_secret_is_unavailable_when_signing_before_a_roll() {
        let store = JwksSecretStore::new(JwksSecretHolder::default());

        assert!(
            store
                .sign(&refresh_claims(1_000))
                .await
                .is_err_and(|error| error.to_string().contains("not available yet"))
        );
    }

    #[tokio::test]
    async fn reports_an_invalid_refresh_token_as_a_rejection() {
        let (store, _secret) = store_with_secret();

        let outcome = store
            .mint_access_token("not.a.valid.token", unix_time(500))
            .await
            .expect("invalid user input is not a system error");

        assert_eq!(
            std::mem::discriminant(&outcome),
            std::mem::discriminant(&MintAccessTokenOutcome::Rejected(
                RefreshTokenRejection::Invalid
            ))
        );
    }

    #[tokio::test]
    async fn reports_a_mint_system_failure_for_corrupt_public_key_material() {
        let mut secret = fresh_p256_secret();
        let refresh_token =
            sign_refresh_token(&secret.current.signing, &refresh_claims(1_000)).await;

        secret.current.public.x = "not-base64url".to_string();

        let holder = JwksSecretHolder::default();

        holder.set(Some(Arc::new(secret)));

        let error = JwksSecretStore::new(holder)
            .mint_access_token(&refresh_token, unix_time(500))
            .await
            .err()
            .expect("corrupt public key material is a system failure");

        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&JwksSecretStoreError::Mint {
                source: TokenSignerError::Verification {
                    source: JwksKeyError::MalformedCompactJws,
                },
            })
        );
    }

    #[tokio::test]
    async fn reports_a_sign_failure_for_unserializable_claims() {
        let (store, _secret) = store_with_secret();

        assert!(
            store
                .sign(&Unserializable)
                .await
                .is_err_and(|error| error.to_string().contains("failed to sign claims"))
        );
    }
}
