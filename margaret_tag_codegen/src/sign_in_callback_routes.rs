use margaret_attributes::canonical_path::CanonicalPath;

#[derive(Debug, Eq, PartialEq)]
pub enum SignInCallbackRoutes<'declarations> {
    Admitted(&'declarations [CanonicalPath]),
    RegisteredWithIssuer,
}
