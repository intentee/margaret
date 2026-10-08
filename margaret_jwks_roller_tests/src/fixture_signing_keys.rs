use async_trait::async_trait;
use tokio::sync::Mutex;
use zeroize::Zeroizing;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::persisted_jwks_secret::PersistedJwksSecret;
use margaret_jwks_roller::signing_keys_document::SigningKeysDocument;
use margaret_jwks_roller::stored_signing_keys::StoredSigningKeys;
use margaret_jwks_roller::stores_signing_keys::StoresSigningKeys;

pub struct FixtureSigningKeys {
    documents: Mutex<Vec<String>>,
}

impl FixtureSigningKeys {
    #[must_use]
    pub fn empty() -> Self {
        Self {
            documents: Mutex::new(Vec::new()),
        }
    }

    #[must_use]
    pub fn holding(document: &str) -> Self {
        Self {
            documents: Mutex::new(vec![document.to_string()]),
        }
    }

    /// # Panics
    ///
    /// Panics when the fixture secret cannot be serialized.
    #[must_use]
    pub fn storing(secret: &JwksSecret) -> Self {
        Self::holding(
            &serde_json::to_string(&PersistedJwksSecret::from_secret(secret))
                .expect("the fixture secret serializes"),
        )
    }

    pub async fn stored_documents(&self) -> usize {
        self.documents.lock().await.len()
    }
}

#[async_trait]
impl StoresSigningKeys for FixtureSigningKeys {
    async fn load_signing_keys(&self) -> anyhow::Result<StoredSigningKeys> {
        Ok(match self.documents.lock().await.last() {
            Some(document) => StoredSigningKeys::Stored(SigningKeysDocument::new(Zeroizing::new(
                document.clone(),
            ))),
            None => StoredSigningKeys::Absent,
        })
    }

    async fn store_signing_keys(&self, document: &SigningKeysDocument) -> anyhow::Result<()> {
        self.documents
            .lock()
            .await
            .push(document.json().to_string());

        Ok(())
    }
}
