use proc_macro2::Ident;

use margaret_container::injected_dependency::InjectedDependency;

pub(crate) enum SessionParameter {
    Injectable {
        dependency: InjectedDependency,
        holder: Ident,
    },
    Route {
        from: String,
        holder: Ident,
    },
}
