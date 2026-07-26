use margaret_http::redirect::Redirect;

pub enum AuthenticatedUserOutcome<User> {
    Anonymous,
    Authenticated(User),
    LoginPageRedirect(Redirect),
}
