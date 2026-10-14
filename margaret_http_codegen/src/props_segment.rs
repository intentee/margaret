use proc_macro2::Ident;

pub(crate) enum PropsSegment {
    CatchAllParameter {
        field: Ident,
        name: String,
        prefix: String,
    },
    Literal {
        decoded: String,
        encoded: String,
    },
    Parameter {
        field: Ident,
        name: String,
        prefix: String,
    },
}
