use crate::asset_href::AssetHref;

#[derive(Clone, Copy)]
pub struct ImageOutput {
    pub(crate) href: AssetHref,
}

impl ImageOutput {
    #[must_use]
    pub const fn new(href: AssetHref) -> Self {
        Self { href }
    }
}
