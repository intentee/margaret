pub enum AuthenticatedUserOutcome<User> {
    Anonymous,
    Authenticated(User),
    LoginPageRedirect { url: String },
}
