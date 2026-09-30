use margaret_attributes::tag::Tag;

#[derive(Clone)]
pub enum FrameworkInjectionRole {
    OAuthClient(Tag),
    TrustedIssuer(Tag),
    Unmarked,
}
