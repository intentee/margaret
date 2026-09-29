use aws_lc_rs::digest;
use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;

#[must_use]
pub fn certificate_thumbprint(algorithm: &'static digest::Algorithm, certificate: &[u8]) -> String {
    Base64UrlUnpadded::encode_string(digest::digest(algorithm, certificate).as_ref())
}
