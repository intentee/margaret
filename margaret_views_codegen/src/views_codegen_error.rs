use thiserror::Error;

use margaret_attributes::attribute_error::AttributeError;

#[derive(Debug, Error)]
pub enum ViewsCodegenError {
    #[error("failed to index the crate: {source}")]
    Index {
        #[from]
        source: AttributeError,
    },

    #[error("#[renders_view] is only supported on structs, but '{view}' is not a struct")]
    ViewNotAStruct { view: String },

    #[error("#[renders_view] '{view}' must also carry #[singleton]")]
    ViewNotSingleton { view: String },

    #[error(
        "#[renders_view] '{view}' must not use #[singleton(provides = ...)]; a view renders as its concrete type"
    )]
    ViewProvidesInterface { view: String },

    #[error("#[renders_view] '{view}' is declared more than once; a view struct maps to exactly one view")]
    DuplicateViewDeclaration { view: String },

    #[error("view '{view}' is missing the 'name' argument")]
    ViewMissingName { view: String },

    #[error(
        "view '{view}' declares the name '{name}', which must be a snake_case identifier usable as a `views` accessor"
    )]
    InvalidViewName { view: String, name: String },

    #[error(
        "views '{first}' and '{second}' both declare the view name '{name}'; each view name may identify at most one view"
    )]
    DuplicateViewName {
        name: String,
        first: String,
        second: String,
    },
}
