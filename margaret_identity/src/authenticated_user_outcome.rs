use margaret_http::redirect::Redirect;
use margaret_http::response::Response;

pub enum AuthenticatedUserOutcome<User> {
    Anonymous,
    Authenticated(User),
    LoginPageRedirect(Redirect),
    Rejected(Response),
}
