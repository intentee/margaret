use std::borrow::Cow;

use form_urlencoded::byte_serialize;
use headers::Authorization;
use headers::authorization::Basic;
use percent_encoding::percent_decode_str;
use zeroize::Zeroizing;

use crate::client_id::ClientId;
use crate::client_secret::ClientSecret;
use crate::oauth_vocabulary_error::OAuthVocabularyError;

fn form_decoded(value: &str) -> Result<String, OAuthVocabularyError> {
    percent_decode_str(&value.replace('+', " "))
        .decode_utf8()
        .map(Cow::into_owned)
        .map_err(|source| OAuthVocabularyError::ClientSecretBasicNotUtf8 { source })
}

fn form_encoded(value: &str) -> String {
    byte_serialize(value.as_bytes()).collect()
}

pub struct ClientSecretBasic {
    pub client_id: String,
    pub secret: Zeroizing<String>,
}

impl ClientSecretBasic {
    /// # Errors
    ///
    /// Returns `OAuthVocabularyError::ClientSecretBasicNotUtf8` when the form-encoded client
    /// identifier or secret does not decode to utf-8.
    pub fn decode(user_id: &str, password: &str) -> Result<Self, OAuthVocabularyError> {
        Ok(Self {
            client_id: form_decoded(user_id)?,
            secret: Zeroizing::new(form_decoded(password)?),
        })
    }

    #[must_use]
    pub fn authorization(client_id: &ClientId, secret: &ClientSecret) -> Authorization<Basic> {
        Authorization::basic(
            &form_encoded(client_id.as_str()),
            &form_encoded(secret.expose()),
        )
    }
}
