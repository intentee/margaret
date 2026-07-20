use crate::include::Include;
use crate::preload::Preload;

#[derive(Clone, Copy)]
pub struct BundleAsset {
    pub(crate) includes: &'static [Include],
    pub(crate) output_preloads: &'static [Preload],
    pub(crate) preloads: &'static [Preload],
}

impl BundleAsset {
    #[must_use]
    pub const fn new(
        includes: &'static [Include],
        output_preloads: &'static [Preload],
        preloads: &'static [Preload],
    ) -> Self {
        Self {
            includes,
            output_preloads,
            preloads,
        }
    }
}
