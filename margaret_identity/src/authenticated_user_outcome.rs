use margaret_http::response::Response;

pub enum AuthenticatedUserOutcome<User> {
    Anonymous,
    Authenticated(User),
    Interrupted(Response),
}
