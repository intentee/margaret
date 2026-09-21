use crate::response_credit_window::RESPONSE_CREDIT_WINDOW;
use crate::web_socket_client_error::WebSocketClientError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ExchangeInterruption {
    ConnectionDropped,
    ConnectionFailed,
    PeerClosed,
    PeerSentUnreadableFrame,
    PeerExceededCredit,
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
            Self::PeerExceededCredit => WebSocketClientError::ExchangeCreditExceeded {
                credit: RESPONSE_CREDIT_WINDOW.frames(),
                url,
            },
        }
    }
}
