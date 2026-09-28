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
    /// Returns `JwksSecretStoreError::Mint`.
    pub async fn mint_access_token(
        &self,
        refresh_token: &str,
        now: DateTime<Utc>,
    ) -> Result<AccessTokenMinting, JwksSecretStoreError> {
        let secret = self.current_secret()?;

        mint_access_token(&secret, refresh_token, now)
            .await
            .map_err(|source| JwksSecretStoreError::Mint { source })
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
            .current
            .signing
            .sign(claims)
            .await
            .map_err(|source| JwksSecretStoreError::Sign { source })
    }

    /// # Errors
    ///
    /// Returns `JwksSecretStoreError::Verify`.
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
    use margaret_jwks_keygen::jwks_secret::JwksSecret;
    use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
    use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
    use margaret_jwks_keygen::token_verification::TokenVerification;
    use margaret_jwks_keygen::verifies_token::VerifiesToken;
    use margaret_token_signer::access_token_minting::AccessTokenMinting;
    use margaret_token_signer::minted_tokens::MintedTokens;
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

    struct StoreWithSecret {
        secret: JwksSecret,
        store: JwksSecretStore,
    }

    fn store_with_secret() -> StoreWithSecret {
        let secret = fresh_p256_secret();
        let holder = JwksSecretHolder::default();

        holder.set(Some(Arc::new(secret.clone())));

        StoreWithSecret {
            secret,
            store: JwksSecretStore::new(holder),
        }
    }

    fn minted_tokens(minting: AccessTokenMinting) -> Option<MintedTokens> {
        match minting {
            AccessTokenMinting::Minted(minted) => Some(minted),
            AccessTokenMinting::ExpiredRefreshToken
            | AccessTokenMinting::MalformedRefreshToken(_)
            | AccessTokenMinting::UnknownRefreshTokenKey => None,
        }
    }

    fn verified_claims(
        result: JwksSecretVerificationResult<RefreshTokenClaims>,
    ) -> Option<RefreshTokenClaims> {
        match result {
            JwksSecretVerificationResult::SignedWithCurrent(claims)
            | JwksSecretVerificationResult::SignedWithPrevious(claims) => Some(claims),
            JwksSecretVerificationResult::Invalid | JwksSecretVerificationResult::Malformed(_) => {
                None
            }
        }
    }

    #[tokio::test]
    async fn mints_an_access_token_from_a_valid_refresh_token() {
        let StoreWithSecret { secret, store } = store_with_secret();
        let refresh_token =
            sign_refresh_token(&secret.current.signing, &refresh_claims(1_000)).await;

        let minting = store
            .mint_access_token(&refresh_token, unix_time(500))
            .await
            .expect("the signing secret is usable");

        let minted = minted_tokens(minting).expect("a valid refresh token mints an access token");

        assert!(!minted.access_token.is_empty());
        assert!(!minted.refresh_token.is_empty());
    }

    #[tokio::test]
    async fn signs_claims_with_the_current_key() {
        let StoreWithSecret { secret, store } = store_with_secret();
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
    fn reports_a_malformed_token_as_a_verification_outcome() {
        let StoreWithSecret { store, .. } = store_with_secret();

        let result = store
            .verify::<RefreshTokenClaims>("not.a.valid.token")
            .expect("the signing secret is usable");

        assert!(verified_claims(result).is_none());
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
    async fn reports_a_malformed_refresh_token_as_a_minting_outcome() {
        let StoreWithSecret { store, .. } = store_with_secret();

        let minting = store
            .mint_access_token("not.a.valid.token", unix_time(500))
            .await
            .expect("the signing secret is usable");

        assert!(minted_tokens(minting).is_none());
    }

    #[tokio::test]
    async fn reports_a_sign_failure_for_unserializable_claims() {
        let StoreWithSecret { store, .. } = store_with_secret();

        assert!(
            store
                .sign(&Unserializable)
                .await
                .is_err_and(|error| error.to_string().contains("failed to sign claims"))
        );
    }

    #[tokio::test]
    async fn reports_a_mint_failure_when_the_current_key_is_corrupt() {
        let StoreWithSecret { secret, store } = store_with_secret();
        let refresh_token =
            sign_refresh_token(&secret.current.signing, &refresh_claims(1_000)).await;
        let mut corrupt = secret;

        corrupt.current.public.x = "invalid @@@".to_string();

        let corrupt_holder = JwksSecretHolder::default();

        corrupt_holder.set(Some(Arc::new(corrupt)));

        let corrupt_store = JwksSecretStore::new(corrupt_holder);

        drop(store);

        assert!(
            corrupt_store
                .mint_access_token(&refresh_token, unix_time(500))
                .await
                .is_err_and(|error| error.to_string().contains("failed to mint an access token"))
        );
    }

    #[tokio::test]
    async fn reports_a_verify_failure_when_the_current_key_is_corrupt() {
        let StoreWithSecret { secret, .. } = store_with_secret();
        let token = sign_refresh_token(&secret.current.signing, &refresh_claims(1_000)).await;
        let mut corrupt = secret;

        corrupt.current.public.x = "invalid @@@".to_string();

        let holder = JwksSecretHolder::default();

        holder.set(Some(Arc::new(corrupt)));

        assert!(
            JwksSecretStore::new(holder)
                .verify::<RefreshTokenClaims>(&token)
                .is_err_and(|error| error.to_string().contains("failed to verify a token"))
        );
    }

    #[tokio::test]
    async fn reports_the_claims_of_a_token_signed_with_the_current_key() {
        let StoreWithSecret { secret, store } = store_with_secret();
        let claims = refresh_claims(1_000);
        let token = sign_refresh_token(&secret.current.signing, &claims).await;

        let result = store
            .verify::<RefreshTokenClaims>(&token)
            .expect("the signing secret is usable");

        assert_eq!(
            verified_claims(result).map(|verified| verified.jti),
            Some(claims.jti)
        );
    }

    #[tokio::test]
    async fn reports_the_claims_of_a_token_signed_with_the_previous_key() {
        let rotated = fresh_p256_secret()
            .rotate()
            .expect("the signing secret rotates");
        let claims = refresh_claims(1_000);
        let token = sign_refresh_token(&rotated.previous.signing, &claims).await;
        let holder = JwksSecretHolder::default();

        holder.set(Some(Arc::new(rotated)));

        let result = JwksSecretStore::new(holder)
            .verify::<RefreshTokenClaims>(&token)
            .expect("the signing secret is usable");

        assert_eq!(
            verified_claims(result).map(|verified| verified.jti),
            Some(claims.jti)
        );
    }
}
