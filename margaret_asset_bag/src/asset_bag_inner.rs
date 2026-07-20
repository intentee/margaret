use dashmap::DashSet;

use crate::external_asset::ExternalAsset;
use crate::include::Include;
use crate::preload::Preload;

#[derive(Default)]
pub(crate) struct AssetBagInner {
    externals: DashSet<ExternalAsset>,
    includes: DashSet<Include>,
    preloads: DashSet<Preload>,
}

impl AssetBagInner {
    pub(crate) fn add_external(&self, external: ExternalAsset) {
        self.externals.insert(external);
    }

    pub(crate) fn add_include(&self, include: Include) {
        self.includes.insert(include);
    }

    pub(crate) fn add_preload(&self, preload: Preload) {
        self.preloads.insert(preload);
    }

    pub(crate) fn sorted_externals(&self) -> Vec<ExternalAsset> {
        let mut externals: Vec<ExternalAsset> =
            self.externals.iter().map(|external| external.clone()).collect();

        externals.sort();

        externals
    }

    pub(crate) fn sorted_includes(&self) -> Vec<Include> {
        let mut includes: Vec<Include> = self.includes.iter().map(|include| *include).collect();

        includes.sort();

        includes
    }

    pub(crate) fn sorted_preloads(&self) -> Vec<Preload> {
        let mut preloads: Vec<Preload> = self.preloads.iter().map(|preload| *preload).collect();

        preloads.sort();

        preloads
    }
}
