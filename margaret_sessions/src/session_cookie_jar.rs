use cookie::Cookie;
use cookie::CookieBuilder;
use cookie::SameSite;

use margaret_http::cookie_changes::CookieChanges;
use margaret_http::request::Request;
use margaret_identity_session::access_token_lifetime_secs::ACCESS_TOKEN_LIFETIME_SECS;

use crate::cookie_domain::CookieDomain;
use crate::session_lifetime_secs::SESSION_LIFETIME_SECS;

fn max_age(lifetime_secs: u32) -> cookie::time::Duration {
    cookie::time::Duration::seconds(i64::from(lifetime_secs))
}

pub enum SessionCookieJar {
    HostOnly,
    SharedWithDomain(CookieDomain),
}

impl SessionCookieJar {
    pub(crate) fn access_cookie(&self, access_token: String) -> Cookie<'static> {
        self.cookie(self.access_name(), access_token)
            .max_age(max_age(ACCESS_TOKEN_LIFETIME_SECS))
            .build()
    }

    pub(crate) fn presented_access_token<'request>(
        &self,
        request: &'request Request,
    ) -> Option<&'request str> {
        request
            .inputs
            .cookies
            .get(self.access_name())
            .map(String::as_str)
    }

    pub(crate) fn presented_secret<'request>(
        &self,
        request: &'request Request,
    ) -> Option<&'request str> {
        request
            .inputs
            .cookies
            .get(self.secret_name())
            .map(String::as_str)
    }

    pub(crate) fn removal(&self) -> CookieChanges {
        CookieChanges {
            cookies: [self.access_name(), self.secret_name()]
                .into_iter()
                .map(|name| {
                    let mut removal = self.cookie(name, String::new()).build();

                    removal.make_removal();

                    removal
                })
                .collect(),
        }
    }

    pub(crate) fn secret_cookie(&self, secret: String) -> Cookie<'static> {
        self.cookie(self.secret_name(), secret)
            .max_age(max_age(SESSION_LIFETIME_SECS))
            .build()
    }

    fn access_name(&self) -> &'static str {
        match self {
            Self::HostOnly => "__Host-margaret-session-access",
            Self::SharedWithDomain(_) => "__Secure-margaret-session-access",
        }
    }

    fn cookie(&self, name: &'static str, value: String) -> CookieBuilder<'static> {
        let builder = Cookie::build((name, value))
            .http_only(true)
            .path("/")
            .same_site(SameSite::Strict)
            .secure(true);

        match self {
            Self::HostOnly => builder,
            Self::SharedWithDomain(domain) => builder.domain(domain.as_str().to_string()),
        }
    }

    fn secret_name(&self) -> &'static str {
        match self {
            Self::HostOnly => "__Host-margaret-session",
            Self::SharedWithDomain(_) => "__Secure-margaret-session",
        }
    }
}
