use syn::Type;

use margaret_syn_type_peeling::option_inner::option_inner;

pub(crate) struct DeclaredColumnType<'field> {
    pub(crate) base: &'field Type,
    pub(crate) declared: &'field Type,
    pub(crate) nullable: bool,
}

impl<'field> DeclaredColumnType<'field> {
    pub(crate) fn of(declared: &'field Type) -> Self {
        match option_inner(declared) {
            Some(base) => Self {
                base,
                declared,
                nullable: true,
            },
            None => Self {
                base: declared,
                declared,
                nullable: false,
            },
        }
    }
}
