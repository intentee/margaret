pub mod reader_cookie;
pub mod reader_realm;
pub mod session_reader_provider;

#[derive(Clone)]
pub struct Reader {
    pub name: String,
}
