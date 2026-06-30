use margaret_macros::http_server;

#[http_server(name = "internal")]
pub struct Internal;
