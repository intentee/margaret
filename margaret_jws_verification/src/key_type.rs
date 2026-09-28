use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) enum KeyType {
    #[serde(rename = "EC")]
    Ec,
    #[serde(rename = "RSA")]
    Rsa,
}
