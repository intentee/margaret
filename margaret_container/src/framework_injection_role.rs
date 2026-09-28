use margaret_attributes::tag::Tag;

#[derive(Clone)]
pub enum FrameworkInjectionRole {
    FrameworkOnly,
    JwksClientStore(Tag),
    JwksServerStore,
    Unmarked,
}
