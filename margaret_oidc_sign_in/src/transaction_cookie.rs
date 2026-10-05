use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use cookie::Cookie;
use cookie::time::Duration;

use margaret_http::host_cookie::host_cookie;
use margaret_identity_session::sign_in_transaction_lifetime_secs::SIGN_IN_TRANSACTION_LIFETIME_SECS;
use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;
use margaret_token_digest::token_digest::TokenDigest;

const TRANSACTION_COOKIE_PREFIX: &str = "__Host-margaret-sign-in-";

pub(crate) struct TransactionCookie {
    pub(crate) name: String,
}

impl TransactionCookie {
    pub(crate) fn of(issuer: &IssuerIdentifier, client_id: &ClientId) -> Self {
        let digest = TokenDigest::of(&format!("{}\0{}", issuer.as_str(), client_id.as_str()));

        Self {
            name: format!(
                "{TRANSACTION_COOKIE_PREFIX}{}",
                Base64UrlUnpadded::encode_string(digest.as_bytes())
            ),
        }
    }

    pub(crate) fn holding(&self, transaction: String) -> Cookie<'static> {
        let mut cookie = host_cookie(self.name.clone(), transaction).build();

        cookie.set_max_age(Duration::seconds(i64::from(
            SIGN_IN_TRANSACTION_LIFETIME_SECS,
        )));

        cookie
    }

    pub(crate) fn removal(&self) -> Cookie<'static> {
        let mut cookie = host_cookie(self.name.clone(), String::new()).build();

        cookie.make_removal();

        cookie
    }
}
