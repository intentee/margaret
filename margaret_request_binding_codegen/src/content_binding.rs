use proc_macro2::Ident;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContentBinding {
    FormFields,
    Json,
    MultipartFieldsAndFiles { files: Ident },
    MultipartFiles { files: Ident },
    Stream { stream: Ident },
}
