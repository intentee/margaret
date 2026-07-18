use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::sync::Arc;

use cookie::Cookie;
use cookie::SameSite;
use dashmap::DashMap;
use dashmap::mapref::entry::Entry;
use http::HeaderMap;
use http::header::COOKIE;

use crate::cookie_attributes::CookieAttributes;
use crate::cookie_config::CookieConfig;
use crate::cookie_jar_error::CookieJarError;
use crate::cookie_operation::CookieOperation;

pub struct CookieJar {
    cookie_config: Arc<CookieConfig>,
    incoming: HashMap<String, String>,
    operations: DashMap<String, CookieOperation>,
}

impl CookieJar {
    pub fn from_headers(
        cookie_config: Arc<CookieConfig>,
        headers: &HeaderMap,
    ) -> Result<Self, CookieJarError> {
        let mut incoming = HashMap::new();

        for raw in headers.get_all(COOKIE) {
            let raw = raw
                .to_str()
                .map_err(|source| CookieJarError::UnreadableHeader { source })?;

            for parsed in Cookie::split_parse_encoded(raw) {
                let cookie = parsed.map_err(|source| CookieJarError::MalformedCookie { source })?;
                let name = cookie.name().to_owned();

                if incoming.insert(name, cookie.value().to_owned()).is_some() {
                    return Err(CookieJarError::DuplicateInRequest {
                        name: cookie.name().to_owned(),
                    });
                }
            }
        }

        Ok(Self {
            cookie_config,
            incoming,
            operations: DashMap::new(),
        })
    }

    pub fn add(
        &self,
        name: &str,
        value: &str,
        attributes: CookieAttributes,
    ) -> Result<(), CookieJarError> {
        if matches!(attributes.same_site, SameSite::None) && !self.cookie_config.secure {
            return Err(CookieJarError::InsecureSameSiteNone {
                name: name.to_owned(),
            });
        }

        match self.operations.entry(name.to_owned()) {
            Entry::Occupied(occupied) => {
                let name = occupied.key().clone();

                Err(match occupied.get() {
                    CookieOperation::Remove => CookieJarError::AlreadyRemoved { name },
                    CookieOperation::Set { .. } => CookieJarError::AlreadySet { name },
                })
            }
            Entry::Vacant(vacant) => {
                vacant.insert(CookieOperation::Set {
                    attributes,
                    value: value.to_owned(),
                });

                Ok(())
            }
        }
    }

    #[must_use]
    pub fn get(&self, name: &str) -> Option<String> {
        match self.operations.get(name) {
            Some(operation) => match operation.value() {
                CookieOperation::Remove => None,
                CookieOperation::Set { value, .. } => Some(value.clone()),
            },
            None => self.incoming.get(name).cloned(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (String, String)> {
        let mut current = self.incoming.clone();

        for operation in &self.operations {
            match operation.value() {
                CookieOperation::Remove => {
                    current.remove(operation.key());
                }
                CookieOperation::Set { value, .. } => {
                    current.insert(operation.key().clone(), value.clone());
                }
            }
        }

        current.into_iter()
    }

    pub fn remove(&self, name: &str) -> Result<(), CookieJarError> {
        match self.operations.entry(name.to_owned()) {
            Entry::Occupied(occupied) => match occupied.get() {
                CookieOperation::Remove => Ok(()),
                CookieOperation::Set { .. } => Err(CookieJarError::AlreadySet {
                    name: name.to_owned(),
                }),
            },
            Entry::Vacant(vacant) => {
                if self.incoming.contains_key(name) {
                    vacant.insert(CookieOperation::Remove);

                    Ok(())
                } else {
                    Err(CookieJarError::NotInRequest {
                        name: name.to_owned(),
                    })
                }
            }
        }
    }

    #[must_use]
    pub fn set_cookie_values(&self) -> Vec<String> {
        let mut staged: Vec<Cookie<'static>> = self
            .operations
            .iter()
            .map(|operation| match operation.value() {
                CookieOperation::Remove => self.removal_cookie(operation.key()),
                CookieOperation::Set { attributes, value } => {
                    self.staged_cookie(operation.key(), value, attributes)
                }
            })
            .collect();

        staged.sort_by(|left, right| left.name().cmp(right.name()));

        staged
            .iter()
            .map(|cookie| cookie.encoded().to_string())
            .collect()
    }

    fn removal_cookie(&self, name: &str) -> Cookie<'static> {
        let mut cookie = Cookie::build((name.to_owned(), String::new()))
            .domain(self.cookie_config.domain.as_str().to_owned())
            .path("/")
            .secure(self.cookie_config.secure)
            .build();

        cookie.make_removal();

        cookie
    }

    fn staged_cookie(
        &self,
        name: &str,
        value: &str,
        attributes: &CookieAttributes,
    ) -> Cookie<'static> {
        Cookie::build((name.to_owned(), value.to_owned()))
            .domain(self.cookie_config.domain.as_str().to_owned())
            .expires(attributes.expiration)
            .http_only(attributes.http_only)
            .path("/")
            .same_site(attributes.same_site)
            .secure(self.cookie_config.secure)
            .build()
    }
}

impl Debug for CookieJar {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("CookieJar").finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use cookie::Expiration;
    use cookie::SameSite;
    use cookie::time::OffsetDateTime;
    use http::HeaderMap;
    use http::HeaderValue;
    use http::header::COOKIE;

    use super::CookieJar;
    use crate::cookie_attributes::CookieAttributes;
    use crate::cookie_config::CookieConfig;
    use crate::cookie_domain::CookieDomain;
    use crate::cookie_jar_error::CookieJarError;

    fn cookie_config(secure: bool) -> Arc<CookieConfig> {
        Arc::new(CookieConfig {
            domain: CookieDomain::parse("example.test").expect("the domain parses"),
            secure,
        })
    }

    fn session_attributes() -> CookieAttributes {
        CookieAttributes {
            expiration: Expiration::Session,
            http_only: true,
            same_site: SameSite::Strict,
        }
    }

    fn empty_jar() -> CookieJar {
        CookieJar::from_headers(cookie_config(true), &HeaderMap::new())
            .expect("an empty header map parses")
    }

    fn jar_from(raw: &'static str) -> CookieJar {
        let mut headers = HeaderMap::new();

        headers.insert(COOKIE, HeaderValue::from_static(raw));

        CookieJar::from_headers(cookie_config(true), &headers).expect("the cookie header parses")
    }

    #[test]
    fn reads_the_cookies_the_request_carried() {
        let jar = jar_from("session=abc; theme=dark");

        assert_eq!(jar.get("session"), Some("abc".to_owned()));
        assert_eq!(jar.get("theme"), Some("dark".to_owned()));
    }

    #[test]
    fn reads_cookies_from_every_cookie_header() {
        let mut headers = HeaderMap::new();

        headers.append(COOKIE, HeaderValue::from_static("session=abc"));
        headers.append(COOKIE, HeaderValue::from_static("theme=dark"));

        let jar = CookieJar::from_headers(cookie_config(true), &headers)
            .expect("the cookie headers parse");

        assert_eq!(jar.get("session"), Some("abc".to_owned()));
        assert_eq!(jar.get("theme"), Some("dark".to_owned()));
    }

    #[test]
    fn reads_no_cookies_without_a_cookie_header() {
        assert_eq!(empty_jar().get("session"), None);
    }

    #[test]
    fn rejects_a_cookie_name_repeated_within_one_header() {
        assert!(matches!(
            CookieJar::from_headers(cookie_config(true), &{
                let mut headers = HeaderMap::new();

                headers.insert(COOKIE, HeaderValue::from_static("session=a; session=b"));

                headers
            })
            .expect_err("a repeated cookie name is rejected"),
            CookieJarError::DuplicateInRequest { name } if name == "session"
        ));
    }

    #[test]
    fn rejects_a_cookie_name_repeated_across_headers() {
        let mut headers = HeaderMap::new();

        headers.append(COOKIE, HeaderValue::from_static("session=a"));
        headers.append(COOKIE, HeaderValue::from_static("session=b"));

        assert!(matches!(
            CookieJar::from_headers(cookie_config(true), &headers)
                .expect_err("a repeated cookie name is rejected"),
            CookieJarError::DuplicateInRequest { name } if name == "session"
        ));
    }

    #[test]
    fn rejects_a_cookie_header_that_is_not_visible_ascii() {
        let mut headers = HeaderMap::new();

        headers.insert(
            COOKIE,
            HeaderValue::from_bytes(&[0xff]).expect("the header value is built"),
        );

        assert!(
            CookieJar::from_headers(cookie_config(true), &headers)
                .expect_err("an unreadable cookie header is rejected")
                .to_string()
                .starts_with("the request cookie header is not visible ASCII text")
        );
    }

    #[test]
    fn rejects_a_malformed_cookie() {
        let mut headers = HeaderMap::new();

        headers.insert(COOKIE, HeaderValue::from_static("novalue"));

        assert!(
            CookieJar::from_headers(cookie_config(true), &headers)
                .expect_err("a malformed cookie is rejected")
                .to_string()
                .starts_with("a cookie in the request header could not be parsed")
        );
    }

    #[test]
    fn reports_the_latest_state_of_a_staged_cookie() {
        let jar = empty_jar();

        jar.add("session", "fresh", session_attributes())
            .expect("the cookie is staged");

        assert_eq!(jar.get("session"), Some("fresh".to_owned()));
    }

    #[test]
    fn reports_a_removed_cookie_as_absent() {
        let jar = jar_from("session=abc");

        jar.remove("session").expect("the cookie is removed");

        assert_eq!(jar.get("session"), None);
    }

    #[test]
    fn rejects_setting_a_cookie_twice() {
        let jar = empty_jar();

        jar.add("session", "first", session_attributes())
            .expect("the cookie is staged");

        assert!(matches!(
            jar.add("session", "second", session_attributes())
                .expect_err("a second set is rejected"),
            CookieJarError::AlreadySet { name } if name == "session"
        ));
    }

    #[test]
    fn rejects_setting_a_cookie_that_was_removed() {
        let jar = jar_from("session=abc");

        jar.remove("session").expect("the cookie is removed");

        assert!(matches!(
            jar.add("session", "again", session_attributes())
                .expect_err("setting a removed cookie is rejected"),
            CookieJarError::AlreadyRemoved { name } if name == "session"
        ));
    }

    #[test]
    fn rejects_same_site_none_without_a_secure_connection() {
        let jar = CookieJar::from_headers(cookie_config(false), &HeaderMap::new())
            .expect("an empty header map parses");

        assert!(matches!(
            jar.add(
                "session",
                "abc",
                CookieAttributes {
                    expiration: Expiration::Session,
                    http_only: true,
                    same_site: SameSite::None,
                },
            )
            .expect_err("an insecure SameSite=None cookie is rejected"),
            CookieJarError::InsecureSameSiteNone { name } if name == "session"
        ));
    }

    #[test]
    fn accepts_same_site_none_over_a_secure_connection() {
        let jar = empty_jar();

        jar.add(
            "session",
            "abc",
            CookieAttributes {
                expiration: Expiration::Session,
                http_only: true,
                same_site: SameSite::None,
            },
        )
        .expect("a secure SameSite=None cookie is accepted");

        assert_eq!(jar.get("session"), Some("abc".to_owned()));
    }

    #[test]
    fn rejects_removing_a_cookie_the_request_did_not_carry() {
        assert!(matches!(
            empty_jar()
                .remove("session")
                .expect_err("removing an absent cookie is rejected"),
            CookieJarError::NotInRequest { name } if name == "session"
        ));
    }

    #[test]
    fn rejects_removing_a_cookie_that_was_set() {
        let jar = empty_jar();

        jar.add("session", "abc", session_attributes())
            .expect("the cookie is staged");

        assert!(matches!(
            jar.remove("session")
                .expect_err("removing a staged cookie is rejected"),
            CookieJarError::AlreadySet { name } if name == "session"
        ));
    }

    #[test]
    fn removes_a_cookie_idempotently() {
        let jar = jar_from("session=abc");

        jar.remove("session").expect("the cookie is removed");
        jar.remove("session")
            .expect("removing the cookie again is accepted");

        assert_eq!(jar.get("session"), None);
    }

    #[test]
    fn lists_the_current_state_of_every_cookie() {
        let jar = jar_from("session=abc; theme=dark");

        jar.remove("theme").expect("the cookie is removed");
        jar.add("visited", "true", session_attributes())
            .expect("the cookie is staged");

        let mut listed: Vec<(String, String)> = jar.iter().collect();

        listed.sort();

        assert_eq!(
            listed,
            vec![
                ("session".to_owned(), "abc".to_owned()),
                ("visited".to_owned(), "true".to_owned()),
            ]
        );
    }

    #[test]
    fn stamps_the_configured_domain_path_and_secure_flag() {
        let jar = empty_jar();

        jar.add("session", "abc", session_attributes())
            .expect("the cookie is staged");

        let emitted = jar.set_cookie_values();

        assert_eq!(emitted.len(), 1);
        assert!(emitted[0].starts_with("session=abc"));
        assert!(emitted[0].contains("Domain=example.test"));
        assert!(emitted[0].contains("Path=/"));
        assert!(emitted[0].contains("Secure"));
        assert!(emitted[0].contains("HttpOnly"));
        assert!(emitted[0].contains("SameSite=Strict"));
    }

    #[test]
    fn omits_the_secure_flag_for_an_insecure_server() {
        let jar = CookieJar::from_headers(cookie_config(false), &HeaderMap::new())
            .expect("an empty header map parses");

        jar.add("session", "abc", session_attributes())
            .expect("the cookie is staged");

        assert!(!jar.set_cookie_values()[0].contains("Secure"));
    }

    #[test]
    fn emits_an_expiring_cookie_for_an_absolute_expiration() {
        let jar = empty_jar();

        jar.add(
            "session",
            "abc",
            CookieAttributes {
                expiration: Expiration::DateTime(
                    OffsetDateTime::from_unix_timestamp(1_700_000_000)
                        .expect("the timestamp is in range"),
                ),
                http_only: true,
                same_site: SameSite::Strict,
            },
        )
        .expect("the cookie is staged");

        assert!(jar.set_cookie_values()[0].contains("Expires="));
    }

    #[test]
    fn emits_a_removal_matching_the_domain_and_path_of_the_original() {
        let jar = jar_from("session=abc");

        jar.remove("session").expect("the cookie is removed");

        let emitted = jar.set_cookie_values();

        assert_eq!(emitted.len(), 1);
        assert!(emitted[0].starts_with("session="));
        assert!(emitted[0].contains("Max-Age=0"));
        assert!(emitted[0].contains("Domain=example.test"));
        assert!(emitted[0].contains("Path=/"));
        assert!(emitted[0].contains("Secure"));
    }

    #[test]
    fn percent_encodes_a_cookie_value() {
        let jar = empty_jar();

        jar.add("session", "a;b", session_attributes())
            .expect("the cookie is staged");

        assert!(jar.set_cookie_values()[0].starts_with("session=a%3Bb"));
    }

    #[test]
    fn emits_staged_cookies_sorted_by_name() {
        let jar = empty_jar();

        jar.add("zulu", "1", session_attributes())
            .expect("the cookie is staged");
        jar.add("alpha", "2", session_attributes())
            .expect("the cookie is staged");

        let emitted = jar.set_cookie_values();

        assert!(emitted[0].starts_with("alpha=2"));
        assert!(emitted[1].starts_with("zulu=1"));
    }

    #[test]
    fn emits_nothing_when_nothing_was_staged() {
        assert!(jar_from("session=abc").set_cookie_values().is_empty());
    }

    #[test]
    fn redacts_its_contents_when_formatted() {
        assert_eq!(format!("{:?}", jar_from("session=abc")), "CookieJar { .. }");
    }
}
