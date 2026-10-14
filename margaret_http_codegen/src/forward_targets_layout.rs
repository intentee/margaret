use proc_macro2::Ident;

pub(crate) struct ForwardTargetsLayout {
    pub(crate) target_module: Ident,
    pub(crate) target_type: Ident,
    pub(crate) targets_module: Ident,
    pub(crate) targets_type: Ident,
}
