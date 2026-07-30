#[derive(Clone, Copy)]
pub enum RequestInput {
    Cookie,
    Form,
    Query,
    Json,
}
