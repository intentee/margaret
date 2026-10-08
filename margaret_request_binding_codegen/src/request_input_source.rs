#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequestInputSource {
    Cookie,
    Form,
    Query,
    Json,
}
