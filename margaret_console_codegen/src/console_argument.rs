use syn::Type;

pub(crate) enum ConsoleArgument {
    Flag {
        name: String,
    },
    Named {
        name: String,
        required: bool,
        value_type: Type,
    },
    Positional {
        id: String,
        required: bool,
        value_type: Type,
    },
}
