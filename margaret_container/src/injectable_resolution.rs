use crate::injected_dependency::InjectedDependency;

pub enum InjectableResolution {
    FrameworkOnly,
    JwksSecretStoreByPath,
    MissingProvider,
    Resolved(InjectedDependency),
    UnsupportedShape,
}
