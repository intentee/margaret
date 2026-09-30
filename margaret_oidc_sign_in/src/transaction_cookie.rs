use aws_lc_rs::digest::Context;
use aws_lc_rs::digest::SHA256;
use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use cookie::Cookie;
use cookie::SameSite;
use cookie::time::Duration;

use margaret_identity_session::sign_in_transaction_lifetime_secs::SIGN_IN_TRANSACTION_LIFETIME_SECS;
use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

const TRANSACTION_COOKIE_PREFIX: &str = "__Host-margaret-sign-in-";

fn host_cookie(name: String, value: String) -> Cookie<'static> {
    Cookie::build((name, value))
        .http_only(true)
        .path("/")
        .same_site(SameSite::Lax)
        .secure(true)
        .build()
}

pub(crate) struct TransactionCookie {
    pub(crate) name: String,
}

impl TransactionCookie {
    pub(crate) fn of(issuer: &IssuerIdentifier, client_id: &ClientId) -> Self {
        let mut context = Context::new(&SHA256);

        context.update(issuer.as_str().as_bytes());
        context.update(&[0]);
        context.update(client_id.as_str().as_bytes());

        Self {
            name: format!(
                "{TRANSACTION_COOKIE_PREFIX}{}",
                Base64UrlUnpadded::encode_string(context.finish().as_ref())
            ),
        }
    }

    pub(crate) fn holding(&self, transaction: String) -> Cookie<'static> {
        let mut cookie = host_cookie(self.name.clone(), transaction);

        cookie.set_max_age(Duration::seconds(i64::from(
            SIGN_IN_TRANSACTION_LIFETIME_SECS,
        )));

        cookie
    }

    pub(crate) fn removal(&self) -> Cookie<'static> {
        let mut cookie = host_cookie(self.name.clone(), String::new());

        cookie.make_removal();

        cookie
    }
}
