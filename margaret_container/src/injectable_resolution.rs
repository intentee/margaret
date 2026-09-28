use crate::injected_dependency::InjectedDependency;

pub enum InjectableResolution {
    FrameworkOnly,
    MissingProvider,
    Resolved(InjectedDependency),
    UnsupportedShape,
}
