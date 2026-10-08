use http::Response;

pub enum IssuerAnswer {
    Oversized { max_bytes: usize },
    Received(Response<Vec<u8>>),
}
