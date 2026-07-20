use crate::asset_href::AssetHref;

#[derive(Clone, Copy)]
pub struct ImageAsset {
    pub(crate) href: AssetHref,
}

impl ImageAsset {
    #[must_use]
    pub const fn new(href: AssetHref) -> Self {
        Self { href }
    }
}
