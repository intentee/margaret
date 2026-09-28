use crate::compact_jws::CompactJws;
use crate::jws_rejection::JwsRejection;

pub enum CompactJwsParsing<'token> {
    Parsed(CompactJws<'token>),
    Rejected(JwsRejection),
}
