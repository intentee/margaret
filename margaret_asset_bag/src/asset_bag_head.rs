use std::sync::Arc;

use maud::Markup;
use maud::Render;
use maud::html;

use crate::asset_bag_inner::AssetBagInner;

pub struct AssetBagHead {
    inner: Arc<AssetBagInner>,
}

impl AssetBagHead {
    pub(crate) fn new(inner: Arc<AssetBagInner>) -> Self {
        Self { inner }
    }
}

impl Render for AssetBagHead {
    fn render(&self) -> Markup {
        html! {
            @for preload in self.inner.sorted_preloads() {
                (preload)
            }
            @for include in self.inner.sorted_includes() {
                (include)
            }
            @for external in self.inner.sorted_externals() {
                (external)
            }
        }
    }
}
