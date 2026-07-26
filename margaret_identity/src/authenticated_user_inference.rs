pub enum AuthenticatedUserInference<User> {
    Anonymous,
    Authenticated(User),
    LoginRequired,
}
