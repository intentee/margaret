#[derive(Clone, Copy)]
pub(crate) enum ProviderRequirement {
    Declared,
    Optional,
    Singleton,
}
