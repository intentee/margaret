use syn::Type;

pub(crate) struct InputParameter {
    pub(crate) name: String,
    pub(crate) ty: Type,
}
