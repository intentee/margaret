use margaret_attributes::canonical_path::CanonicalPath;

use crate::uri_selected_provider::UriSelectedProvider;

pub enum FrameworkProvider {
    Unit(CanonicalPath),
    UriSelected(UriSelectedProvider),
}

impl FrameworkProvider {
    pub(crate) fn key(&self) -> &CanonicalPath {
        match self {
            FrameworkProvider::Unit(path) => path,
            FrameworkProvider::UriSelected(provider) => &provider.trait_path,
        }
    }
}
