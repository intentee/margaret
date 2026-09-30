use aws_lc_rs::digest::SHA256;
use aws_lc_rs::digest::SHA256_OUTPUT_LEN;
use aws_lc_rs::digest::digest;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TokenDigest {
    digest: [u8; SHA256_OUTPUT_LEN],
}

impl TokenDigest {
    #[must_use]
    pub fn of(token: &str) -> Self {
        let mut computed = [0; SHA256_OUTPUT_LEN];

        computed.copy_from_slice(digest(&SHA256, token.as_bytes()).as_ref());

        Self { digest: computed }
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8; SHA256_OUTPUT_LEN] {
        &self.digest
    }
}
