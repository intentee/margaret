use http::HeaderName;

#[derive(Debug)]
pub(crate) struct Header {
    pub(crate) name: HeaderName,
    pub(crate) value: String,
}
