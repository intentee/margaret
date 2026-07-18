use cookie::Expiration;
use cookie::SameSite;
use cookie::time::OffsetDateTime;

use margaret_cookie_jar::cookie_attributes::CookieAttributes;
use margaret_cookie_jar::cookie_jar::CookieJar;

use crate::access_token_claims_signed::AccessTokenClaimsSigned;
use crate::cookie_name_access_token::COOKIE_NAME_ACCESS_TOKEN;
use crate::cookie_name_refresh_token::COOKIE_NAME_REFRESH_TOKEN;
use crate::identity_session_error::IdentitySessionError;
use crate::refresh_token_claims_signed::RefreshTokenClaimsSigned;

fn session_attributes(exp: i64) -> Result<CookieAttributes, IdentitySessionError> {
    Ok(CookieAttributes {
        expiration: Expiration::DateTime(
            OffsetDateTime::from_unix_timestamp(exp)
                .map_err(|source| IdentitySessionError::CookieExpiration { exp, source })?,
        ),
        http_only: true,
        same_site: SameSite::Strict,
    })
}

#[derive(Clone)]
pub struct SessionCookieManager;

impl SessionCookieManager {
    pub fn clear_access_token(&self, cookie_jar: &CookieJar) -> Result<(), IdentitySessionError> {
        cookie_jar
            .remove(COOKIE_NAME_ACCESS_TOKEN)
            .map_err(|source| IdentitySessionError::CookieStaging { source })
    }

    pub fn clear_refresh_token(&self, cookie_jar: &CookieJar) -> Result<(), IdentitySessionError> {
        cookie_jar
            .remove(COOKIE_NAME_REFRESH_TOKEN)
            .map_err(|source| IdentitySessionError::CookieStaging { source })
    }

    pub fn issue_access_token(
        &self,
        cookie_jar: &CookieJar,
        AccessTokenClaimsSigned { exp, signed_claims }: AccessTokenClaimsSigned,
    ) -> Result<(), IdentitySessionError> {
        cookie_jar
            .add(
                COOKIE_NAME_ACCESS_TOKEN,
                &signed_claims,
                session_attributes(exp)?,
            )
            .map_err(|source| IdentitySessionError::CookieStaging { source })
    }

    pub fn issue_refresh_token(
        &self,
        cookie_jar: &CookieJar,
        RefreshTokenClaimsSigned { exp, signed_claims }: RefreshTokenClaimsSigned,
    ) -> Result<(), IdentitySessionError> {
        cookie_jar
            .add(
                COOKIE_NAME_REFRESH_TOKEN,
                &signed_claims,
                session_attributes(exp)?,
            )
            .map_err(|source| IdentitySessionError::CookieStaging { source })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use http::HeaderMap;
    use http::HeaderValue;
    use http::header::COOKIE;

    use margaret_cookie_jar::cookie_config::CookieConfig;
    use margaret_cookie_jar::cookie_domain::CookieDomain;
    use margaret_cookie_jar::cookie_jar::CookieJar;

    use super::SessionCookieManager;
    use crate::access_token_claims_signed::AccessTokenClaimsSigned;
    use crate::cookie_name_access_token::COOKIE_NAME_ACCESS_TOKEN;
    use crate::cookie_name_refresh_token::COOKIE_NAME_REFRESH_TOKEN;
    use crate::identity_session_error::IdentitySessionError;
    use crate::refresh_token_claims_signed::RefreshTokenClaimsSigned;

    fn cookie_jar(raw: &'static str) -> CookieJar {
        let mut headers = HeaderMap::new();

        headers.insert(COOKIE, HeaderValue::from_static(raw));

        CookieJar::from_headers(
            Arc::new(CookieConfig {
                domain: CookieDomain::parse("example.test").expect("the domain parses"),
                secure: true,
            }),
            &headers,
        )
        .expect("the cookie header parses")
    }

    #[test]
    fn issues_an_access_token_cookie_that_expires() {
        let cookie_jar = cookie_jar("other=1");

        SessionCookieManager
            .issue_access_token(
                &cookie_jar,
                AccessTokenClaimsSigned {
                    exp: 1_700_000_000,
                    signed_claims: "signed-access".to_string(),
                },
            )
            .expect("the access token cookie is staged");

        let emitted = cookie_jar.set_cookie_values();

        assert_eq!(
            cookie_jar.get(COOKIE_NAME_ACCESS_TOKEN).as_deref(),
            Some("signed-access")
        );
        assert!(emitted[0].contains("Domain=example.test"));
        assert!(emitted[0].contains("HttpOnly"));
        assert!(emitted[0].contains("Path=/"));
        assert!(emitted[0].contains("SameSite=Strict"));
        assert!(emitted[0].contains("Secure"));
        assert!(emitted[0].contains("Expires="));
    }

    #[test]
    fn issues_a_refresh_token_cookie_that_expires() {
        let cookie_jar = cookie_jar("other=1");

        SessionCookieManager
            .issue_refresh_token(
                &cookie_jar,
                RefreshTokenClaimsSigned {
                    exp: 1_700_000_000,
                    signed_claims: "signed-refresh".to_string(),
                },
            )
            .expect("the refresh token cookie is staged");

        assert_eq!(
            cookie_jar.get(COOKIE_NAME_REFRESH_TOKEN).as_deref(),
            Some("signed-refresh")
        );
        assert!(cookie_jar.set_cookie_values()[0].contains("Expires="));
    }

    #[test]
    fn clears_the_access_token_cookie() {
        let cookie_jar = cookie_jar("access_token=signed-access");

        SessionCookieManager
            .clear_access_token(&cookie_jar)
            .expect("the access token cookie is cleared");

        assert_eq!(cookie_jar.get(COOKIE_NAME_ACCESS_TOKEN), None);
        assert!(cookie_jar.set_cookie_values()[0].contains("Max-Age=0"));
    }

    #[test]
    fn clears_the_refresh_token_cookie() {
        let cookie_jar = cookie_jar("refresh_token=signed-refresh");

        SessionCookieManager
            .clear_refresh_token(&cookie_jar)
            .expect("the refresh token cookie is cleared");

        assert_eq!(cookie_jar.get(COOKIE_NAME_REFRESH_TOKEN), None);
        assert!(cookie_jar.set_cookie_values()[0].contains("Max-Age=0"));
    }

    #[test]
    fn reports_a_clear_that_the_request_cannot_satisfy() {
        let error = SessionCookieManager
            .clear_access_token(&cookie_jar("other=1"))
            .expect_err("clearing a cookie the request did not carry is reported");

        assert!(matches!(
            error,
            IdentitySessionError::CookieStaging { ref source }
                if source.to_string().contains("is not in the request")
        ));
    }

    #[test]
    fn reports_a_refresh_token_clear_that_the_request_cannot_satisfy() {
        let error = SessionCookieManager
            .clear_refresh_token(&cookie_jar("other=1"))
            .expect_err("clearing a cookie the request did not carry is reported");

        assert!(matches!(
            error,
            IdentitySessionError::CookieStaging { ref source }
                if source.to_string().contains("is not in the request")
        ));
    }

    #[test]
    fn reports_an_access_token_that_was_already_issued() {
        let cookie_jar = cookie_jar("other=1");
        let claims = || AccessTokenClaimsSigned {
            exp: 1_700_000_000,
            signed_claims: "signed-access".to_string(),
        };

        SessionCookieManager
            .issue_access_token(&cookie_jar, claims())
            .expect("the access token cookie is staged");

        assert!(matches!(
            SessionCookieManager
                .issue_access_token(&cookie_jar, claims())
                .expect_err("issuing the access token twice is reported"),
            IdentitySessionError::CookieStaging { ref source }
                if source.to_string().contains("is already set")
        ));
    }

    #[test]
    fn reports_a_refresh_token_that_was_already_issued() {
        let cookie_jar = cookie_jar("other=1");
        let claims = || RefreshTokenClaimsSigned {
            exp: 1_700_000_000,
            signed_claims: "signed-refresh".to_string(),
        };

        SessionCookieManager
            .issue_refresh_token(&cookie_jar, claims())
            .expect("the refresh token cookie is staged");

        assert!(matches!(
            SessionCookieManager
                .issue_refresh_token(&cookie_jar, claims())
                .expect_err("issuing the refresh token twice is reported"),
            IdentitySessionError::CookieStaging { ref source }
                if source.to_string().contains("is already set")
        ));
    }

    #[test]
    fn rejects_an_out_of_range_access_token_expiration() {
        let error = SessionCookieManager
            .issue_access_token(
                &cookie_jar("other=1"),
                AccessTokenClaimsSigned {
                    exp: i64::MAX,
                    signed_claims: "signed-access".to_string(),
                },
            )
            .expect_err("an out-of-range expiration is rejected");

        assert!(matches!(
            error,
            IdentitySessionError::CookieExpiration { exp, .. } if exp == i64::MAX
        ));
        assert!(error.to_string().contains("out of range"));
    }

    #[test]
    fn rejects_an_out_of_range_refresh_token_expiration() {
        let error = SessionCookieManager
            .issue_refresh_token(
                &cookie_jar("other=1"),
                RefreshTokenClaimsSigned {
                    exp: i64::MAX,
                    signed_claims: "signed-refresh".to_string(),
                },
            )
            .expect_err("an out-of-range expiration is rejected");

        assert!(matches!(
            error,
            IdentitySessionError::CookieExpiration { exp, .. } if exp == i64::MAX
        ));
    }
}
