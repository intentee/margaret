use margaret_http::response_continuation::ResponseContinuation;

pub enum AuthenticatedUserOutcome<User> {
    Anonymous,
    Authenticated(User),
    Interrupted(ResponseContinuation),
}
