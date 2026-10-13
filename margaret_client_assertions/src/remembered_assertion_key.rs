use aws_lc_rs::digest::Context;
use aws_lc_rs::digest::SHA256;
use aws_lc_rs::digest::SHA256_OUTPUT_LEN;

use margaret_token_digest::token_digest::TokenDigest;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RememberedAssertionKey {
    digest: [u8; SHA256_OUTPUT_LEN],
}

impl RememberedAssertionKey {
    #[must_use]
    pub fn of(client_id: &str, assertion: TokenDigest) -> Self {
        let mut context = Context::new(&SHA256);
        let mut digest = [0; SHA256_OUTPUT_LEN];

        context.update(assertion.as_bytes());
        context.update(client_id.as_bytes());
        digest.copy_from_slice(context.finish().as_ref());

        Self { digest }
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8; SHA256_OUTPUT_LEN] {
        &self.digest
    }
}

#[cfg(test)]
mod tests {
    use margaret_token_digest::token_digest::TokenDigest;

    use super::RememberedAssertionKey;

    #[test]
    fn keys_the_same_assertion_apart_for_different_clients() {
        let assertion = TokenDigest::of("an-assertion");

        assert_ne!(
            RememberedAssertionKey::of("portal", assertion),
            RememberedAssertionKey::of("service", assertion)
        );
    }

    #[test]
    fn keys_an_assertion_of_a_client_the_same_way_every_time() {
        let assertion = TokenDigest::of("an-assertion");

        assert_eq!(
            RememberedAssertionKey::of("portal", assertion),
            RememberedAssertionKey::of("portal", assertion)
        );
    }
}
