use proc_macro2::Ident;

use crate::props_prefix::PropsPrefix;

pub(crate) enum PropsSegment {
    CatchAllParameter {
        field: Ident,
        name: String,
        prefix: PropsPrefix,
    },
    Literal {
        decoded: String,
        encoded: String,
    },
    Parameter {
        field: Ident,
        name: String,
        prefix: PropsPrefix,
    },
}
