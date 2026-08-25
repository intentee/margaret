use syn::Path;

use margaret_model::on_delete::OnDelete;

pub(crate) enum OnDeleteArgument {
    Known(OnDelete),
    Unknown(Path),
}

impl OnDeleteArgument {
    pub(crate) fn of(declared: Option<Path>) -> Self {
        match declared {
            None => OnDeleteArgument::Known(OnDelete::NoAction),
            Some(path) if path.is_ident("cascade") => OnDeleteArgument::Known(OnDelete::Cascade),
            Some(path) if path.is_ident("restrict") => OnDeleteArgument::Known(OnDelete::Restrict),
            Some(path) if path.is_ident("set_null") => OnDeleteArgument::Known(OnDelete::SetNull),
            Some(path) if path.is_ident("set_default") => {
                OnDeleteArgument::Known(OnDelete::SetDefault)
            }
            Some(path) => OnDeleteArgument::Unknown(path),
        }
    }
}
