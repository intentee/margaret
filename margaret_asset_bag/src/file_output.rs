use crate::asset_href::AssetHref;

#[derive(Clone, Copy)]
pub struct FileOutput {
    pub(crate) href: AssetHref,
}

impl FileOutput {
    #[must_use]
    pub const fn new(href: AssetHref) -> Self {
        Self { href }
    }
}
