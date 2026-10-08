use margaret_attributes::tag::Tag;

#[derive(Clone)]
pub enum FrameworkInjectionRole {
    OAuthClient(Tag),
    Runner,
    TrustedIssuer(Tag),
    Unmarked,
}
