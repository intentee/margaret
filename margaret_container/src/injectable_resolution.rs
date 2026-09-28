use crate::injected_dependency::InjectedDependency;

pub enum InjectableResolution {
    JwksSecretStoreByPath,
    MissingProvider,
    Resolved(InjectedDependency),
    UnsupportedShape,
}
