use bytes::Bytes;
use http_body_util::combinators::UnsyncBoxBody;

pub(crate) type RequestBody = UnsyncBoxBody<Bytes, std::io::Error>;
