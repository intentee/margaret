use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeclarationAnchorError {
    #[error(
        "'{path}' is a #[singleton], but #[{attribute}] declares data and needs a plain struct to anchor it"
    )]
    DeclaredAsSingleton {
        attribute: &'static str,
        path: String,
    },

    #[error("#[{attribute}] must be placed on a struct, but '{path}' is not a struct")]
    NotAStruct {
        attribute: &'static str,
        path: String,
    },

    #[error("'{path}' anchors {declarations} declarations; each declaration needs its own struct")]
    SharedAnchor { declarations: usize, path: String },
}
