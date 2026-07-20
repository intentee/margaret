use crate::asset_href::AssetHref;

#[derive(Clone, Copy)]
pub struct StaticAsset {
    pub(crate) href: AssetHref,
}

impl StaticAsset {
    #[must_use]
    pub const fn new(href: AssetHref) -> Self {
        Self { href }
    }
}
