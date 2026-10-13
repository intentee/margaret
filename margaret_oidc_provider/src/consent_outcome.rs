use margaret_http::response::Response;

pub enum ConsentOutcome {
    Redirected(Response),
    Unknown,
}
