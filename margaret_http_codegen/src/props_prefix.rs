use proc_macro2::Ident;

pub(crate) enum PropsPrefix {
    Prefixed {
        module: Ident,
        text: String,
        type_ident: Ident,
    },
    Unprefixed,
}
