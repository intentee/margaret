use crate::injected_dependency::InjectedDependency;

pub enum InjectableResolution {
    MissingProvider,
    Resolved(InjectedDependency),
    UnsupportedShape,
}
