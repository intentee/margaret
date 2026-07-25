use margaret_attributes::tag::Tag;

pub enum FrameworkInjectionRole {
    JwksClientStore(Tag),
    JwksServerStore,
    Unmarked,
}
