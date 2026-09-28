use margaret_attributes::tag::Tag;

#[derive(Clone)]
pub enum FrameworkInjectionRole {
    JwksClientStore(Tag),
    JwksServerStore,
    Unmarked,
}
