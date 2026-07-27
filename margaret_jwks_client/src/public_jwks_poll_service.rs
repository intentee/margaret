use std::collections::BTreeSet;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use futures_util::StreamExt;
use log::error;
use reqwest::Client;
use reqwest::Response;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use trzcina::TickContext;
use trzcina::Ticker;

use margaret_jwks_endpoint::provides_endpoint::ProvidesEndpoint;
use margaret_jwks_keygen::public_jwks::PublicJwks;

use crate::jwks_client_error::JwksClientError;
use crate::jwks_poll_interval_after_ready::JWKS_POLL_INTERVAL_AFTER_READY;
use crate::jwks_poll_interval_before_ready::JWKS_POLL_INTERVAL_BEFORE_READY;
use crate::public_jwks_holder::PublicJwksHolder;

const MAX_JWKS_BYTES: usize = 1024 * 1024;
const JWKS_CONTENT_TYPE: reqwest::header::HeaderValue =
    reqwest::header::HeaderValue::from_static("application/jwk-set+json");

fn endpoint_is_secure(url: &url::Url) -> bool {
    url.scheme() == "https"
        && url.username().is_empty()
        && url.password().is_none()
        && url.query().is_none()
        && url.fragment().is_none()
}

fn require_secure_endpoint(url: url::Url) -> Result<url::Url, JwksClientError> {
    if endpoint_is_secure(&url) {
        Ok(url)
    } else {
        Err(JwksClientError::InvalidEndpoint)
    }
}

fn require_jwks_content_type(
    content_type: Option<&reqwest::header::HeaderValue>,
) -> Result<(), JwksClientError> {
    if content_type == Some(&JWKS_CONTENT_TYPE) {
        Ok(())
    } else {
        Err(JwksClientError::InvalidContentType)
    }
}

fn require_allowed_declared_size(content_length: Option<u64>) -> Result<(), JwksClientError> {
    if content_length.is_some_and(|length| length > MAX_JWKS_BYTES as u64) {
        Err(JwksClientError::DocumentTooLarge {
            limit: MAX_JWKS_BYTES,
        })
    } else {
        Ok(())
    }
}

struct JwksDocument {
    bytes: Vec<u8>,
}

impl JwksDocument {
    fn empty() -> Self {
        Self { bytes: Vec::new() }
    }

    fn extend(&mut self, chunk: &[u8]) -> Result<(), JwksClientError> {
        if chunk.len() > MAX_JWKS_BYTES - self.bytes.len() {
            return Err(JwksClientError::DocumentTooLarge {
                limit: MAX_JWKS_BYTES,
            });
        }

        self.bytes.extend_from_slice(chunk);

        Ok(())
    }

    fn parse(self) -> Result<PublicJwks, JwksClientError> {
        let jwks: PublicJwks = serde_json::from_slice(&self.bytes)
            .map_err(|source| JwksClientError::InvalidDocument { source })?;

        unique_key_ids(&jwks)?;

        Ok(jwks)
    }
}

async fn read_public_jwks_response(response: Response) -> Result<PublicJwks, JwksClientError> {
    require_jwks_content_type(response.headers().get(reqwest::header::CONTENT_TYPE))?;
    require_allowed_declared_size(response.content_length())?;

    let mut document = JwksDocument::empty();
    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(JwksClientError::DocumentFetch)?;
        document.extend(&chunk)?;
    }

    document.parse()
}

fn unique_key_ids(jwks: &PublicJwks) -> Result<(), JwksClientError> {
    let mut ids = BTreeSet::new();

    for key in &jwks.keys {
        if !ids.insert(key.kid.as_str()) {
            return Err(JwksClientError::DuplicateKeyId {
                kid: key.kid.clone(),
            });
        }
    }

    Ok(())
}

pub struct PublicJwksPollService {
    pub endpoint_provider: Arc<dyn ProvidesEndpoint>,
    pub http_client: Client,
    pub public_jwks_holder: PublicJwksHolder,
}

impl PublicJwksPollService {
    pub async fn fetch_public_jwks(&self) -> Result<PublicJwks, JwksClientError> {
        let jwks_url = self
            .endpoint_provider
            .provide()
            .await
            .map_err(JwksClientError::EndpointResolution)
            .and_then(require_secure_endpoint)?;

        let response = self
            .http_client
            .get(jwks_url)
            .send()
            .await
            .and_then(Response::error_for_status)
            .map_err(JwksClientError::DocumentFetch)?;
        read_public_jwks_response(response).await
    }
}

#[async_trait]
impl Ticker for PublicJwksPollService {
    fn tick_interval(&self) -> Duration {
        JWKS_POLL_INTERVAL_BEFORE_READY
    }

    async fn handle_tick(
        &mut self,
        cancellation_token: CancellationToken,
        _tick_context: TickContext,
    ) -> Result<()> {
        match self.fetch_public_jwks().await {
            Ok(public_jwks) => self.public_jwks_holder.set(Some(Arc::new(public_jwks))),
            Err(error) => error!("Unable to fetch the jwks document from the issuer: {error}"),
        }

        if self.public_jwks_holder.is_ready() {
            tokio::select! {
                biased;
                () = cancellation_token.cancelled() => {}
                () = sleep(JWKS_POLL_INTERVAL_AFTER_READY) => {}
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use margaret_jwks_keygen::curve::Curve;
    use margaret_jwks_keygen::jwk_public::JwkPublic;
    use margaret_jwks_keygen::key_type::KeyType;
    use margaret_jwks_keygen::key_use::KeyUse;
    use reqwest::header::HeaderValue;
    use url::Url;

    use super::JWKS_CONTENT_TYPE;
    use super::JwksDocument;
    use super::MAX_JWKS_BYTES;
    use super::endpoint_is_secure;
    use super::read_public_jwks_response;
    use super::require_allowed_declared_size;
    use super::require_jwks_content_type;
    use super::require_secure_endpoint;

    fn parses(value: &str) -> Url {
        Url::parse(value).expect("the test URL parses")
    }

    #[test]
    fn accepts_an_https_endpoint_with_a_path() {
        assert!(endpoint_is_secure(&parses(
            "https://issuer.example/.well-known/jwks.json"
        )));
    }

    #[test]
    fn rejects_every_endpoint_form_that_changes_authority_or_resource_identity() {
        for endpoint in [
            "http://issuer.example/.well-known/jwks.json",
            "https://user@issuer.example/.well-known/jwks.json",
            "https://:secret@issuer.example/.well-known/jwks.json",
            "https://issuer.example/.well-known/jwks.json?tenant=other",
            "https://issuer.example/.well-known/jwks.json#other",
        ] {
            assert!(
                !endpoint_is_secure(&parses(endpoint)),
                "accepted insecure endpoint {endpoint}"
            );
        }
    }

    #[test]
    fn converts_an_insecure_endpoint_into_a_hard_error() {
        assert!(require_secure_endpoint(parses("http://issuer.example/jwks.json")).is_err());
        assert!(require_secure_endpoint(parses("https://issuer.example/jwks.json")).is_ok());
    }

    #[test]
    fn requires_the_exact_jwks_media_type() {
        assert!(require_jwks_content_type(Some(&JWKS_CONTENT_TYPE)).is_ok());
        assert!(
            require_jwks_content_type(Some(&HeaderValue::from_static("application/json"))).is_err()
        );
        assert!(require_jwks_content_type(None).is_err());
    }

    #[test]
    fn rejects_an_oversized_declared_document() {
        assert!(require_allowed_declared_size(None).is_ok());
        assert!(require_allowed_declared_size(Some(MAX_JWKS_BYTES as u64)).is_ok());
        assert!(require_allowed_declared_size(Some(MAX_JWKS_BYTES as u64 + 1)).is_err());
    }

    #[test]
    fn rejects_an_oversized_streamed_document_before_extending_it() {
        let mut document = JwksDocument::empty();

        document
            .extend(&vec![0; MAX_JWKS_BYTES])
            .expect("the exact limit is accepted");

        assert!(document.extend(&[0]).is_err());
    }

    #[test]
    fn parses_a_strict_document_and_rejects_ambiguity() {
        let key = JwkPublic {
            crv: Curve::P256,
            kid: "duplicate".to_string(),
            kty: KeyType::Ec,
            use_: KeyUse::Signature,
            x: "x".to_string(),
            y: "y".to_string(),
        };
        let bytes = serde_json::to_vec(&serde_json::json!({
            "keys": [key.clone(), key],
        }))
        .expect("the duplicate document serializes");
        let mut duplicate = JwksDocument::empty();

        duplicate.extend(&bytes).expect("the small document fits");

        assert!(duplicate.parse().is_err());

        let mut malformed = JwksDocument::empty();

        malformed.extend(b"not json").expect("the small input fits");

        assert!(malformed.parse().is_err());
    }

    fn response(
        content_type: Option<HeaderValue>,
        content_length: Option<u64>,
        body: reqwest::Body,
    ) -> reqwest::Response {
        let mut builder = http::Response::builder();

        if let Some(content_type) = content_type {
            builder = builder.header(reqwest::header::CONTENT_TYPE, content_type);
        }
        if let Some(content_length) = content_length {
            builder = builder.header(reqwest::header::CONTENT_LENGTH, content_length);
        }

        builder
            .body(body)
            .expect("the response fixture is valid")
            .into()
    }

    #[tokio::test]
    async fn reads_only_exact_bounded_jwks_responses() {
        let valid = response(
            Some(JWKS_CONTENT_TYPE),
            None,
            reqwest::Body::from(r#"{"keys":[]}"#),
        );

        assert!(read_public_jwks_response(valid).await.is_ok());

        let absent_content_type = response(None, None, reqwest::Body::from(r#"{"keys":[]}"#));

        assert!(
            read_public_jwks_response(absent_content_type)
                .await
                .is_err()
        );

        let oversized_declaration = response(
            Some(JWKS_CONTENT_TYPE),
            Some(MAX_JWKS_BYTES as u64 + 1),
            reqwest::Body::from(vec![0; MAX_JWKS_BYTES + 1]),
        );

        assert!(
            read_public_jwks_response(oversized_declaration)
                .await
                .is_err()
        );

        let oversized_stream = response(
            Some(JWKS_CONTENT_TYPE),
            None,
            reqwest::Body::wrap_stream(futures_util::stream::once(async {
                Ok::<_, std::io::Error>(Bytes::from(vec![0; MAX_JWKS_BYTES + 1]))
            })),
        );

        assert!(read_public_jwks_response(oversized_stream).await.is_err());

        let failed_stream = response(
            Some(JWKS_CONTENT_TYPE),
            None,
            reqwest::Body::wrap_stream(futures_util::stream::once(async {
                Err::<Bytes, _>(std::io::Error::from(std::io::ErrorKind::ConnectionReset))
            })),
        );

        assert!(read_public_jwks_response(failed_stream).await.is_err());
    }
}
