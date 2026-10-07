use syn::Ident;

pub(crate) fn is_identifier(name: &str) -> bool {
    syn::parse_str::<Ident>(name).is_ok()
}
