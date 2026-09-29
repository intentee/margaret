use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) enum KeyType {
    #[serde(rename = "EC")]
    Ec,
    #[serde(rename = "oct")]
    Oct,
    #[serde(rename = "RSA")]
    Rsa,
}
