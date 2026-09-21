/// How many unread responses one exchange may hold before the client treats its consumer as
/// stalled. This is a stall detector rather than a flow control window: a consumer that is draining
/// its stream never approaches it, while a consumer that has stopped polling loses its own exchange
/// instead of stalling every other exchange on the connection.
pub const RESPONSE_BACKLOG_LIMIT: usize = 32;
