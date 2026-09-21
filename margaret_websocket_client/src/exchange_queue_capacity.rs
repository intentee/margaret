use crate::response_credit_window::RESPONSE_CREDIT_WINDOW;

/// Flow control exempts the error frame, as HTTP/2 exempts `RST_STREAM`, so the queue keeps one
/// slot beyond the window for it. A rejection is then deliverable even when the window is spent.
const EXEMPT_ERROR_FRAME: usize = 1;

/// How many frames one exchange buffers: everything the peer was granted, plus the exempt error
/// frame. The peer may not run further ahead than its grant, so this queue can never be overrun by
/// a peer that honours the protocol.
pub const EXCHANGE_QUEUE_CAPACITY: usize = RESPONSE_CREDIT_WINDOW + EXEMPT_ERROR_FRAME;
