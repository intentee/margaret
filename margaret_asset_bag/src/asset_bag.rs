use std::sync::Arc;

use crate::asset_bag_head::AssetBagHead;
use crate::asset_bag_inner::AssetBagInner;
use crate::bundle_asset::BundleAsset;
use crate::external_asset::ExternalAsset;
use crate::image_asset::ImageAsset;
use crate::static_asset::StaticAsset;

#[derive(Clone, Default)]
pub struct AssetBag {
    inner: Arc<AssetBagInner>,
}

impl AssetBag {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&self, BundleAsset { includes, preloads, .. }: BundleAsset) {
        for include in includes {
            self.inner.add_include(*include);
        }

        for preload in preloads {
            self.inner.add_preload(*preload);
        }
    }

    #[must_use]
    pub fn file(&self, StaticAsset { href }: StaticAsset) -> String {
        href.url()
    }

    #[must_use]
    pub fn head(&self) -> AssetBagHead {
        AssetBagHead::new(self.inner.clone())
    }

    #[must_use]
    pub fn image(&self, ImageAsset { href }: ImageAsset) -> String {
        href.url()
    }

    pub fn preload(
        &self,
        BundleAsset {
            output_preloads,
            preloads,
            ..
        }: BundleAsset,
    ) {
        for preload in output_preloads {
            self.inner.add_preload(*preload);
        }

        for preload in preloads {
            self.inner.add_preload(*preload);
        }
    }

    pub fn script(&self, url: &str) {
        self.inner
            .add_external(ExternalAsset::Script(url.to_string()));
    }

    pub fn stylesheet(&self, url: &str) {
        self.inner
            .add_external(ExternalAsset::Stylesheet(url.to_string()));
    }
}

#[cfg(test)]
mod tests {
    use maud::Render;

    use super::AssetBag;
    use crate::asset_href::AssetHref;
    use crate::bundle_asset::BundleAsset;
    use crate::image_asset::ImageAsset;
    use crate::include::Include;
    use crate::preload::Preload;
    use crate::static_asset::StaticAsset;

    const INCLUDES: &[Include] = &[
        Include::Script(AssetHref::Local("assets/app_ABC12345.js")),
        Include::Stylesheet(AssetHref::Local("assets/app_ABC12345.css")),
    ];
    const OUTPUT_PRELOADS: &[Preload] = &[
        Preload::Module(AssetHref::Local("assets/app_ABC12345.js")),
        Preload::Style(AssetHref::Local("assets/app_ABC12345.css")),
    ];
    const PRELOADS: &[Preload] = &[
        Preload::Module(AssetHref::Local("assets/chunk_ABC12345.js")),
        Preload::Font(AssetHref::Absolute("https://fonts.example/font.woff2")),
    ];

    fn bundle() -> BundleAsset {
        BundleAsset::new(INCLUDES, OUTPUT_PRELOADS, PRELOADS)
    }

    fn head_of(asset_bag: &AssetBag) -> String {
        asset_bag.head().render().into_string()
    }

    #[test]
    fn add_emits_the_bundle_includes_and_transitive_preloads() {
        let asset_bag = AssetBag::new();

        asset_bag.add(bundle());

        assert_eq!(
            head_of(&asset_bag),
            concat!(
                "<link rel=\"preload\" href=\"https://fonts.example/font.woff2\" as=\"font\" crossorigin>",
                "<link rel=\"modulepreload\" href=\"/assets/chunk_ABC12345.js\">",
                "<script async src=\"/assets/app_ABC12345.js\" type=\"module\"></script>",
                "<link rel=\"stylesheet\" href=\"/assets/app_ABC12345.css\">",
            )
        );
    }

    #[test]
    fn add_is_idempotent_for_the_same_bundle() {
        let asset_bag = AssetBag::new();

        asset_bag.add(bundle());
        asset_bag.add(bundle());

        assert_eq!(
            head_of(&asset_bag),
            concat!(
                "<link rel=\"preload\" href=\"https://fonts.example/font.woff2\" as=\"font\" crossorigin>",
                "<link rel=\"modulepreload\" href=\"/assets/chunk_ABC12345.js\">",
                "<script async src=\"/assets/app_ABC12345.js\" type=\"module\"></script>",
                "<link rel=\"stylesheet\" href=\"/assets/app_ABC12345.css\">",
            )
        );
    }

    #[test]
    fn preload_emits_the_outputs_as_preloads_without_includes() {
        let asset_bag = AssetBag::new();

        asset_bag.preload(bundle());

        assert_eq!(
            head_of(&asset_bag),
            concat!(
                "<link rel=\"preload\" href=\"https://fonts.example/font.woff2\" as=\"font\" crossorigin>",
                "<link rel=\"modulepreload\" href=\"/assets/app_ABC12345.js\">",
                "<link rel=\"modulepreload\" href=\"/assets/chunk_ABC12345.js\">",
                "<link rel=\"preload\" href=\"/assets/app_ABC12345.css\" as=\"style\">",
            )
        );
    }

    #[test]
    fn file_resolves_a_static_asset_to_its_hashed_url() {
        let asset_bag = AssetBag::new();

        assert_eq!(
            asset_bag.file(StaticAsset::new(AssetHref::Local("assets/data_ABC12345.bin"))),
            "/assets/data_ABC12345.bin"
        );
    }

    #[test]
    fn image_resolves_an_image_asset_to_its_hashed_url() {
        let asset_bag = AssetBag::new();

        assert_eq!(
            asset_bag.image(ImageAsset::new(AssetHref::Local("assets/logo_ABC12345.png"))),
            "/assets/logo_ABC12345.png"
        );
    }

    #[test]
    fn script_and_stylesheet_emit_external_assets_after_the_includes() {
        let asset_bag = AssetBag::new();

        asset_bag.add(bundle());
        asset_bag.script("https://challenges.example/api.js");
        asset_bag.stylesheet("https://fonts.example/inter.css");

        assert_eq!(
            head_of(&asset_bag),
            concat!(
                "<link rel=\"preload\" href=\"https://fonts.example/font.woff2\" as=\"font\" crossorigin>",
                "<link rel=\"modulepreload\" href=\"/assets/chunk_ABC12345.js\">",
                "<script async src=\"/assets/app_ABC12345.js\" type=\"module\"></script>",
                "<link rel=\"stylesheet\" href=\"/assets/app_ABC12345.css\">",
                "<script async defer src=\"https://challenges.example/api.js\"></script>",
                "<link rel=\"stylesheet\" href=\"https://fonts.example/inter.css\">",
            )
        );
    }

    #[test]
    fn an_empty_bag_renders_no_head_markup() {
        assert_eq!(head_of(&AssetBag::new()), "");
    }
}
