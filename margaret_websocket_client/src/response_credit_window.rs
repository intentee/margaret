use margaret_websocket_envelope::credit_grant::CreditGrant;

/// How many response frames a peer may send for one exchange before the consumer grants more. This
/// is the exchange's flow control window: the peer may not run further ahead than this, so what the
/// exchange buffers is bounded by it, and it is granted back one frame at a time as the consumer
/// drains the stream.
pub const RESPONSE_CREDIT_WINDOW: CreditGrant = CreditGrant::from_frames(32);
