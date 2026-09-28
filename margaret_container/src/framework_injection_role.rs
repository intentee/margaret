use margaret_attributes::tag::Tag;

#[derive(Clone)]
pub enum FrameworkInjectionRole {
    TokenIssuerClient(Tag),
    Unmarked,
}
