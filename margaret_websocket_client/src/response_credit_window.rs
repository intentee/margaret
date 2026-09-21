/// How many response frames a peer may send for one exchange before the consumer replenishes it.
/// This is the exchange's flow control window: the peer may not run further ahead than this, so the
/// client's queue for that exchange is bounded by it, and it is replenished one frame at a time as
/// the consumer drains the stream.
pub const RESPONSE_CREDIT_WINDOW: usize = 32;
