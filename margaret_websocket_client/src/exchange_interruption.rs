use crate::response_backlog_limit::RESPONSE_BACKLOG_LIMIT;
use crate::web_socket_client_error::WebSocketClientError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ExchangeInterruption {
    ConnectionDropped,
    ConnectionFailed,
    PeerClosed,
    PeerSentUnreadableFrame,
    ResponseBacklogExceeded,
}

impl ExchangeInterruption {
    pub(crate) fn into_error(self, url: &str) -> WebSocketClientError {
        let url = url.to_string();

        match self {
            Self::ConnectionDropped => WebSocketClientError::ExchangeConnectionDropped { url },
            Self::ConnectionFailed => WebSocketClientError::ExchangeConnectionFailed { url },
            Self::PeerClosed => WebSocketClientError::ExchangePeerClosed { url },
            Self::PeerSentUnreadableFrame => {
                WebSocketClientError::ExchangeProtocolViolation { url }
            }
            Self::ResponseBacklogExceeded => WebSocketClientError::ExchangeBacklogExceeded {
                limit: RESPONSE_BACKLOG_LIMIT,
                url,
            },
        }
    }
}
