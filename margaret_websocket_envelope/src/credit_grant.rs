use serde::Deserialize;
use serde::Serialize;

use crate::web_socket_envelope_error::WebSocketEnvelopeError;

/// How many response frames one exchange may be granted. The representation carries the protocol
/// maximum, so a grant outside it cannot be built, and every grant fits a `tokio` semaphore on
/// every supported target.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(into = "usize", try_from = "usize")]
pub struct CreditGrant {
    frames: u16,
}

impl CreditGrant {
    pub const MAXIMUM: usize = u16::MAX as usize;

    #[must_use]
    pub const fn from_frames(frames: u16) -> Self {
        Self { frames }
    }

    #[must_use]
    pub const fn frames(self) -> usize {
        self.frames as usize
    }
}

impl From<CreditGrant> for usize {
    fn from(grant: CreditGrant) -> Self {
        grant.frames()
    }
}

impl TryFrom<usize> for CreditGrant {
    type Error = WebSocketEnvelopeError;

    fn try_from(credit: usize) -> Result<Self, Self::Error> {
        match u16::try_from(credit) {
            Ok(frames) => Ok(Self::from_frames(frames)),
            Err(_) => Err(WebSocketEnvelopeError::CreditGrantOutOfRange {
                credit,
                maximum: Self::MAXIMUM,
            }),
        }
    }
}
