use std::net::Ipv4Addr;
use std::str::FromStr;

use crate::cookie_jar_error::CookieJarError;

const MAXIMUM_DOMAIN_LENGTH: usize = 255;
const MAXIMUM_LABEL_LENGTH: usize = 63;

fn validate_label(label: &str) -> Result<(), CookieJarError> {
    if label.is_empty() {
        return Err(CookieJarError::DomainLabelEmpty);
    }

    if label.len() > MAXIMUM_LABEL_LENGTH {
        return Err(CookieJarError::DomainLabelTooLong {
            label: label.to_owned(),
        });
    }

    if label.starts_with('-') || label.ends_with('-') {
        return Err(CookieJarError::DomainLabelHyphenBoundary {
            label: label.to_owned(),
        });
    }

    for character in label.chars() {
        if !character.is_ascii_alphanumeric() && character != '-' {
            return Err(CookieJarError::DomainLabelInvalidCharacter {
                character,
                label: label.to_owned(),
            });
        }
    }

    Ok(())
}

#[derive(Debug)]
pub struct CookieDomain {
    value: String,
}

impl CookieDomain {
    pub fn parse(value: &str) -> Result<Self, CookieJarError> {
        if value.is_empty() {
            return Err(CookieJarError::DomainEmpty);
        }

        if value.len() > MAXIMUM_DOMAIN_LENGTH {
            return Err(CookieJarError::DomainTooLong {
                length: value.len(),
            });
        }

        if value.starts_with('.') {
            return Err(CookieJarError::DomainStartsWithDot {
                value: value.to_owned(),
            });
        }

        if Ipv4Addr::from_str(value).is_ok() {
            return Err(CookieJarError::DomainIsIpAddress {
                value: value.to_owned(),
            });
        }

        for label in value.split('.') {
            validate_label(label)?;
        }

        Ok(Self {
            value: value.to_ascii_lowercase(),
        })
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::CookieDomain;
    use crate::cookie_jar_error::CookieJarError;

    #[test]
    fn accepts_a_multi_label_domain() {
        assert_eq!(
            CookieDomain::parse("example.test")
                .expect("a multi label domain is accepted")
                .as_str(),
            "example.test"
        );
    }

    #[test]
    fn accepts_localhost_so_local_development_keeps_working() {
        assert_eq!(
            CookieDomain::parse("localhost")
                .expect("localhost is accepted")
                .as_str(),
            "localhost"
        );
    }

    #[test]
    fn lowercases_the_domain() {
        assert_eq!(
            CookieDomain::parse("Example.TEST")
                .expect("an upper case domain is accepted")
                .as_str(),
            "example.test"
        );
    }

    #[test]
    fn accepts_a_label_containing_a_hyphen() {
        assert_eq!(
            CookieDomain::parse("my-app.example.test")
                .expect("an inner hyphen is accepted")
                .as_str(),
            "my-app.example.test"
        );
    }

    #[test]
    fn rejects_an_empty_domain() {
        assert_eq!(
            CookieDomain::parse("")
                .expect_err("an empty domain is rejected")
                .to_string(),
            "the cookie domain is empty"
        );
    }

    #[test]
    fn rejects_a_domain_longer_than_the_dns_limit() {
        let value = "a".repeat(256);

        assert!(matches!(
            CookieDomain::parse(&value).expect_err("an over-long domain is rejected"),
            CookieJarError::DomainTooLong { length } if length == 256
        ));
    }

    #[test]
    fn rejects_a_domain_starting_with_a_dot() {
        assert!(matches!(
            CookieDomain::parse(".example.test").expect_err("a leading dot is rejected"),
            CookieJarError::DomainStartsWithDot { value } if value == ".example.test"
        ));
    }

    #[test]
    fn rejects_an_ip_address_literal() {
        assert!(matches!(
            CookieDomain::parse("127.0.0.1").expect_err("an ip literal is rejected"),
            CookieJarError::DomainIsIpAddress { value } if value == "127.0.0.1"
        ));
    }

    #[test]
    fn rejects_an_empty_label() {
        assert_eq!(
            CookieDomain::parse("example..test")
                .expect_err("an empty label is rejected")
                .to_string(),
            "the cookie domain contains an empty label"
        );
    }

    #[test]
    fn rejects_a_label_longer_than_the_dns_limit() {
        let label = "a".repeat(64);

        assert!(matches!(
            CookieDomain::parse(&format!("{label}.test"))
                .expect_err("an over-long label is rejected"),
            CookieJarError::DomainLabelTooLong { label: rejected } if rejected.len() == 64
        ));
    }

    #[test]
    fn rejects_a_label_starting_with_a_hyphen() {
        assert!(matches!(
            CookieDomain::parse("-example.test").expect_err("a leading hyphen is rejected"),
            CookieJarError::DomainLabelHyphenBoundary { label } if label == "-example"
        ));
    }

    #[test]
    fn rejects_a_label_ending_with_a_hyphen() {
        assert!(matches!(
            CookieDomain::parse("example-.test").expect_err("a trailing hyphen is rejected"),
            CookieJarError::DomainLabelHyphenBoundary { label } if label == "example-"
        ));
    }

    #[test]
    fn rejects_a_domain_carrying_a_header_terminator() {
        assert!(matches!(
            CookieDomain::parse("example.test\r\nx-injected: 1")
                .expect_err("a header terminator is rejected"),
            CookieJarError::DomainLabelInvalidCharacter { character, .. } if character == '\r'
        ));
    }
}
