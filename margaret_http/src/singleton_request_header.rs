use http::HeaderName;
use http::header::AUTHORIZATION;
use http::header::CONTENT_LENGTH;
use http::header::CONTENT_TYPE;
use http::header::COOKIE;
use http::header::EXPECT;
use http::header::HOST;
use http::header::ORIGIN;
use http::header::PROXY_AUTHORIZATION;
use http::header::REFERER;
use http::header::SEC_WEBSOCKET_KEY;
use http::header::SEC_WEBSOCKET_VERSION;
use http::header::USER_AGENT;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SingletonRequestHeader {
    Authorization,
    ContentLength,
    ContentType,
    Cookie,
    Expect,
    Host,
    Origin,
    ProxyAuthorization,
    Referer,
    SecWebSocketKey,
    SecWebSocketVersion,
    UserAgent,
}

impl SingletonRequestHeader {
    pub(crate) fn classify(name: &HeaderName) -> Option<Self> {
        if name == AUTHORIZATION {
            Some(Self::Authorization)
        } else if name == CONTENT_LENGTH {
            Some(Self::ContentLength)
        } else if name == CONTENT_TYPE {
            Some(Self::ContentType)
        } else if name == COOKIE {
            Some(Self::Cookie)
        } else if name == EXPECT {
            Some(Self::Expect)
        } else if name == HOST {
            Some(Self::Host)
        } else if name == ORIGIN {
            Some(Self::Origin)
        } else if name == PROXY_AUTHORIZATION {
            Some(Self::ProxyAuthorization)
        } else if name == REFERER {
            Some(Self::Referer)
        } else if name == SEC_WEBSOCKET_KEY {
            Some(Self::SecWebSocketKey)
        } else if name == SEC_WEBSOCKET_VERSION {
            Some(Self::SecWebSocketVersion)
        } else if name == USER_AGENT {
            Some(Self::UserAgent)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use http::HeaderName;
    use http::header::ACCEPT_ENCODING;
    use http::header::AUTHORIZATION;
    use http::header::CONTENT_LENGTH;
    use http::header::CONTENT_TYPE;
    use http::header::COOKIE;
    use http::header::EXPECT;
    use http::header::HOST;
    use http::header::ORIGIN;
    use http::header::PROXY_AUTHORIZATION;
    use http::header::REFERER;
    use http::header::SEC_WEBSOCKET_KEY;
    use http::header::SEC_WEBSOCKET_VERSION;
    use http::header::USER_AGENT;

    use super::SingletonRequestHeader;

    #[test]
    fn classifies_every_singleton_request_header() {
        for (name, expected) in [
            (AUTHORIZATION, SingletonRequestHeader::Authorization),
            (CONTENT_LENGTH, SingletonRequestHeader::ContentLength),
            (CONTENT_TYPE, SingletonRequestHeader::ContentType),
            (COOKIE, SingletonRequestHeader::Cookie),
            (EXPECT, SingletonRequestHeader::Expect),
            (HOST, SingletonRequestHeader::Host),
            (ORIGIN, SingletonRequestHeader::Origin),
            (
                PROXY_AUTHORIZATION,
                SingletonRequestHeader::ProxyAuthorization,
            ),
            (REFERER, SingletonRequestHeader::Referer),
            (SEC_WEBSOCKET_KEY, SingletonRequestHeader::SecWebSocketKey),
            (
                SEC_WEBSOCKET_VERSION,
                SingletonRequestHeader::SecWebSocketVersion,
            ),
            (USER_AGENT, SingletonRequestHeader::UserAgent),
        ] {
            assert_eq!(SingletonRequestHeader::classify(&name), Some(expected));
        }
    }

    #[test]
    fn treats_a_list_valued_header_as_repeatable() {
        assert_eq!(SingletonRequestHeader::classify(&ACCEPT_ENCODING), None);
    }

    #[test]
    fn treats_a_consumer_defined_header_as_repeatable() {
        let name = HeaderName::from_static("x-consumer-defined");

        assert_eq!(SingletonRequestHeader::classify(&name), None);
    }
}
