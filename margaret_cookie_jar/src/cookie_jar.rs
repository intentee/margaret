use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Formatter;

use cookie::Cookie;
use dashmap::DashMap;
use dashmap::mapref::entry::Entry;
use http::HeaderMap;
use http::header::COOKIE;

use crate::cookie_jar_error::CookieJarError;
use crate::cookie_operation::CookieOperation;

fn removal_cookie(name: &str) -> Cookie<'static> {
    let mut cookie = Cookie::build((name.to_owned(), String::new()))
        .path("/")
        .build();

    cookie.make_removal();

    cookie
}

pub struct CookieJar {
    incoming: HashMap<String, String>,
    operations: DashMap<String, CookieOperation>,
}

impl CookieJar {
    pub fn from_headers(headers: &HeaderMap) -> Result<Self, CookieJarError> {
        let mut incoming = HashMap::new();

        if let Some(raw) = headers.get(COOKIE) {
            let raw = raw
                .to_str()
                .map_err(|source| CookieJarError::UnreadableHeader { source })?;

            for parsed in Cookie::split_parse_encoded(raw) {
                let cookie = parsed.map_err(|source| CookieJarError::MalformedCookie { source })?;

                incoming
                    .entry(cookie.name().to_owned())
                    .or_insert_with(|| cookie.value().to_owned());
            }
        }

        Ok(Self {
            incoming,
            operations: DashMap::new(),
        })
    }

    pub fn add(&self, cookie: Cookie<'static>) -> Result<(), CookieJarError> {
        let name = cookie.name().to_owned();

        match self.operations.entry(name) {
            Entry::Occupied(occupied) => {
                let name = occupied.key().clone();

                Err(match occupied.get() {
                    CookieOperation::Remove => CookieJarError::AlreadyRemoved { name },
                    CookieOperation::Set { .. } => CookieJarError::AlreadySet { name },
                })
            }
            Entry::Vacant(vacant) => {
                vacant.insert(CookieOperation::Set { cookie });

                Ok(())
            }
        }
    }

    #[must_use]
    pub fn get(&self, name: &str) -> Option<String> {
        match self.operations.get(name) {
            Some(operation) => match operation.value() {
                CookieOperation::Remove => None,
                CookieOperation::Set { cookie } => Some(cookie.value().to_owned()),
            },
            None => self.incoming.get(name).cloned(),
        }
    }

    #[must_use]
    pub fn into_set_cookie_values(self) -> Vec<String> {
        let mut staged: Vec<Cookie<'static>> = self
            .operations
            .into_iter()
            .map(|(name, operation)| match operation {
                CookieOperation::Remove => removal_cookie(&name),
                CookieOperation::Set { cookie } => cookie,
            })
            .collect();

        staged.sort_by(|left, right| left.name().cmp(right.name()));

        staged
            .iter()
            .map(|cookie| cookie.encoded().to_string())
            .collect()
    }

    pub fn iter(&self) -> impl Iterator<Item = (String, String)> {
        let mut current = self.incoming.clone();

        for operation in &self.operations {
            match operation.value() {
                CookieOperation::Remove => {
                    current.remove(operation.key());
                }
                CookieOperation::Set { cookie } => {
                    current.insert(operation.key().clone(), cookie.value().to_owned());
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
}

impl Debug for CookieJar {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("CookieJar").finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use cookie::Cookie;
    use http::HeaderMap;
    use http::HeaderValue;
    use http::header::COOKIE;

    use super::CookieJar;
    use crate::cookie_jar_error::CookieJarError;

    fn cookie(name: &str, value: &str) -> Cookie<'static> {
        Cookie::build((name.to_owned(), value.to_owned())).build()
    }

    fn empty_jar() -> CookieJar {
        CookieJar::from_headers(&HeaderMap::new()).expect("an empty jar is built")
    }

    fn jar_from(raw: &str) -> CookieJar {
        let mut headers = HeaderMap::new();

        headers.insert(
            COOKIE,
            HeaderValue::from_str(raw).expect("the cookie header value is valid"),
        );

        CookieJar::from_headers(&headers).expect("the cookie header parses")
    }

    #[test]
    fn redacts_its_contents_when_formatted() {
        let formatted = format!("{:?}", jar_from("session=secret"));

        assert_eq!(formatted, "CookieJar { .. }");
        assert!(!formatted.contains("secret"));
    }

    #[test]
    fn reads_the_cookies_the_request_carried() {
        let jar = jar_from("session=abc; theme=dark");

        assert_eq!(jar.get("session"), Some("abc".to_owned()));
        assert_eq!(jar.get("theme"), Some("dark".to_owned()));
    }

    #[test]
    fn reads_no_cookies_without_a_cookie_header() {
        assert_eq!(empty_jar().get("session"), None);
    }

    #[test]
    fn keeps_the_first_of_two_cookies_sharing_a_name() {
        assert_eq!(
            jar_from("session=first; session=second").get("session"),
            Some("first".to_owned())
        );
    }

    #[test]
    fn decodes_a_percent_encoded_cookie_value() {
        assert_eq!(
            jar_from("session=a%3Bb").get("session"),
            Some("a;b".to_owned())
        );
    }

    #[test]
    fn rejects_a_cookie_header_that_is_not_visible_ascii() {
        let mut headers = HeaderMap::new();

        headers.insert(
            COOKIE,
            HeaderValue::from_bytes(b"session=\xff").expect("the header value is built"),
        );

        let error =
            CookieJar::from_headers(&headers).expect_err("a non ASCII cookie header is rejected");

        assert!(
            error
                .to_string()
                .starts_with("the request cookie header is not visible ASCII text")
        );
    }

    #[test]
    fn rejects_a_malformed_cookie_header() {
        let mut headers = HeaderMap::new();

        headers.insert(COOKIE, HeaderValue::from_static("no-equals-sign"));

        let error =
            CookieJar::from_headers(&headers).expect_err("a malformed cookie header is rejected");

        assert!(
            error
                .to_string()
                .starts_with("a cookie in the request header could not be parsed")
        );
    }

    #[test]
    fn reads_a_staged_cookie_as_the_latest_state() {
        let jar = jar_from("session=old");

        jar.add(cookie("issued", "new"))
            .expect("the cookie is staged");

        assert_eq!(jar.get("issued"), Some("new".to_owned()));
        assert_eq!(jar.get("session"), Some("old".to_owned()));
    }

    #[test]
    fn reads_a_removed_cookie_as_absent() {
        let jar = jar_from("session=abc");

        jar.remove("session").expect("the removal is staged");

        assert_eq!(jar.get("session"), None);
    }

    #[test]
    fn iterates_the_latest_state() {
        let jar = jar_from("kept=1; dropped=2");

        jar.remove("dropped").expect("the removal is staged");
        jar.add(cookie("added", "3")).expect("the cookie is staged");

        let mut current: Vec<(String, String)> = jar.iter().collect();

        current.sort();

        assert_eq!(
            current,
            vec![
                ("added".to_owned(), "3".to_owned()),
                ("kept".to_owned(), "1".to_owned()),
            ]
        );
    }

    #[test]
    fn emits_a_set_cookie_value_for_a_staged_cookie() {
        let jar = empty_jar();

        jar.add(cookie("session", "abc"))
            .expect("the cookie is staged");

        assert_eq!(jar.into_set_cookie_values(), vec!["session=abc".to_owned()]);
    }

    #[test]
    fn percent_encodes_a_set_cookie_value() {
        let jar = empty_jar();

        jar.add(cookie("session", "a;b"))
            .expect("the cookie is staged");

        assert_eq!(
            jar.into_set_cookie_values(),
            vec!["session=a%3Bb".to_owned()]
        );
    }

    #[test]
    fn emits_a_removal_for_a_received_cookie() {
        let jar = jar_from("session=abc");

        jar.remove("session").expect("the removal is staged");

        let emitted = jar.into_set_cookie_values();

        assert_eq!(emitted.len(), 1);
        assert!(emitted[0].starts_with("session="));
        assert!(emitted[0].contains("Max-Age=0"));
        assert!(emitted[0].contains("Path=/"));
    }

    #[test]
    fn emits_nothing_when_nothing_was_staged() {
        assert!(jar_from("session=abc").into_set_cookie_values().is_empty());
    }

    #[test]
    fn orders_the_emitted_cookies_by_name() {
        let jar = empty_jar();

        jar.add(cookie("second", "2"))
            .expect("the cookie is staged");
        jar.add(cookie("first", "1")).expect("the cookie is staged");

        assert_eq!(
            jar.into_set_cookie_values(),
            vec!["first=1".to_owned(), "second=2".to_owned()]
        );
    }

    #[test]
    fn rejects_setting_a_cookie_that_is_already_set() {
        let jar = empty_jar();

        jar.add(cookie("session", "abc"))
            .expect("the cookie is staged");

        let error = jar
            .add(cookie("session", "def"))
            .expect_err("the second set is rejected");

        assert!(matches!(
            error,
            CookieJarError::AlreadySet { ref name } if name == "session"
        ));
        assert_eq!(
            error.to_string(),
            "the cookie `session` is already set in this request"
        );
    }

    #[test]
    fn rejects_setting_a_cookie_that_is_already_removed() {
        let jar = jar_from("session=abc");

        jar.remove("session").expect("the removal is staged");

        let error = jar
            .add(cookie("session", "def"))
            .expect_err("setting a removed cookie is rejected");

        assert!(matches!(
            error,
            CookieJarError::AlreadyRemoved { ref name } if name == "session"
        ));
        assert_eq!(
            error.to_string(),
            "the cookie `session` is already removed in this request"
        );
    }

    #[test]
    fn rejects_removing_a_cookie_that_is_already_set() {
        let jar = jar_from("session=abc");

        jar.add(cookie("session", "def"))
            .expect("the cookie is staged");

        let error = jar
            .remove("session")
            .expect_err("removing a set cookie is rejected");

        assert!(matches!(
            error,
            CookieJarError::AlreadySet { ref name } if name == "session"
        ));
    }

    #[test]
    fn allows_removing_a_cookie_twice() {
        let jar = jar_from("session=abc");

        jar.remove("session").expect("the removal is staged");
        jar.remove("session")
            .expect("removing an already removed cookie is allowed");

        assert_eq!(jar.into_set_cookie_values().len(), 1);
    }

    #[test]
    fn rejects_removing_a_cookie_the_request_did_not_carry() {
        let error = jar_from("other=1")
            .remove("session")
            .expect_err("removing an unreceived cookie is rejected");

        assert!(matches!(
            error,
            CookieJarError::NotInRequest { ref name } if name == "session"
        ));
        assert_eq!(
            error.to_string(),
            "the cookie `session` is not in the request"
        );
    }
}
