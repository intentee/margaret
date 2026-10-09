use form_urlencoded::byte_serialize;
use headers::Authorization;
use headers::authorization::Basic;

use crate::client_secret::ClientSecret;

fn form_encoded(value: &str) -> String {
    byte_serialize(value.as_bytes()).collect()
}

#[must_use]
pub fn client_secret_basic(client_id: &str, secret: &ClientSecret) -> Authorization<Basic> {
    Authorization::basic(&form_encoded(client_id), &form_encoded(secret.expose()))
}
