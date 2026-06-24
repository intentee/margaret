use syn::Type;

pub(crate) struct ConsoleArgument {
    pub(crate) cli_name: String,
    pub(crate) value_type: Type,
}
