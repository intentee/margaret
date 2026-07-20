use std::sync::Arc;

use crate::asset_bag_head::AssetBagHead;
use crate::asset_bag_inner::AssetBagInner;
use crate::asset_resolution::AssetResolution;
use crate::external_asset::ExternalAsset;
use crate::file_output::FileOutput;
use crate::has_script::HasScript;
use crate::has_stylesheet::HasStylesheet;
use crate::image_output::ImageOutput;
use crate::registers_includes::RegistersIncludes;

#[derive(Clone, Default)]
pub struct AssetBag {
    inner: Arc<AssetBagInner>,
}

impl AssetBag {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<TBundle: RegistersIncludes, TImage, TFile>(
        &self,
        AssetResolution { bundle, .. }: AssetResolution<TBundle, TImage, TFile>,
    ) {
        for include in bundle.includes() {
            self.inner.add_include(include);
        }

        for preload in bundle.preloads() {
            self.inner.add_preload(*preload);
        }
    }

    pub fn external_script(&self, url: &str) {
        self.inner
            .add_external(ExternalAsset::Script(url.to_string()));
    }

    pub fn external_stylesheet(&self, url: &str) {
        self.inner
            .add_external(ExternalAsset::Stylesheet(url.to_string()));
    }

    #[must_use]
    pub fn file<TBundle, TImage>(
        &self,
        AssetResolution { file, .. }: AssetResolution<TBundle, TImage, FileOutput>,
    ) -> String {
        file.href.url()
    }

    #[must_use]
    pub fn head(&self) -> AssetBagHead {
        AssetBagHead::new(self.inner.clone())
    }

    #[must_use]
    pub fn image<TBundle, TFile>(
        &self,
        AssetResolution { image, .. }: AssetResolution<TBundle, ImageOutput, TFile>,
    ) -> String {
        image.href.url()
    }

    pub fn preload<TBundle: RegistersIncludes, TImage, TFile>(
        &self,
        AssetResolution { bundle, .. }: AssetResolution<TBundle, TImage, TFile>,
    ) {
        for preload in bundle.output_preloads() {
            self.inner.add_preload(preload);
        }

        for preload in bundle.preloads() {
            self.inner.add_preload(*preload);
        }
    }

    pub fn script<TBundle: HasScript, TImage, TFile>(
        &self,
        AssetResolution { bundle, .. }: AssetResolution<TBundle, TImage, TFile>,
    ) {
        self.inner.add_include(bundle.script_include());

        for preload in bundle.script_preloads() {
            self.inner.add_preload(*preload);
        }
    }

    pub fn stylesheet<TBundle: HasStylesheet, TImage, TFile>(
        &self,
        AssetResolution { bundle, .. }: AssetResolution<TBundle, TImage, TFile>,
    ) {
        self.inner.add_include(bundle.stylesheet_include());

        for preload in bundle.stylesheet_preloads() {
            self.inner.add_preload(*preload);
        }
    }
}

#[cfg(test)]
mod tests {
    use maud::Render;

    use super::AssetBag;
    use crate::absent::Absent;
    use crate::asset_href::AssetHref;
    use crate::asset_resolution::AssetResolution;
    use crate::file_output::FileOutput;
    use crate::image_output::ImageOutput;
    use crate::preload::Preload;
    use crate::script_bundle::ScriptBundle;
    use crate::script_stylesheet_bundle::ScriptStylesheetBundle;
    use crate::stylesheet_bundle::StylesheetBundle;

    const PRELOADS: &[Preload] = &[
        Preload::Module(AssetHref::Local("assets/chunk_ABC12345.js")),
        Preload::Font(AssetHref::Absolute("https://fonts.example/font.woff2")),
    ];

    fn script_stylesheet() -> AssetResolution<ScriptStylesheetBundle, Absent, Absent> {
        AssetResolution::new(
            ScriptStylesheetBundle::new(
                AssetHref::Local("assets/app_ABC12345.js"),
                AssetHref::Local("assets/app_ABC12345.css"),
                PRELOADS,
            ),
            Absent,
            Absent,
        )
    }

    fn script_only() -> AssetResolution<ScriptBundle, Absent, Absent> {
        AssetResolution::new(
            ScriptBundle::new(AssetHref::Local("assets/solo_ABC12345.js"), PRELOADS),
            Absent,
            Absent,
        )
    }

    fn stylesheet_only() -> AssetResolution<StylesheetBundle, Absent, Absent> {
        AssetResolution::new(
            StylesheetBundle::new(AssetHref::Local("assets/theme_ABC12345.css"), PRELOADS),
            Absent,
            Absent,
        )
    }

    fn head_of(asset_bag: &AssetBag) -> String {
        asset_bag.head().render().into_string()
    }

    #[test]
    fn add_registers_the_script_and_stylesheet_includes_with_transitive_preloads() {
        let asset_bag = AssetBag::new();

        asset_bag.add(script_stylesheet());

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
    fn add_is_idempotent_for_the_same_resolution() {
        let asset_bag = AssetBag::new();

        asset_bag.add(script_stylesheet());
        asset_bag.add(script_stylesheet());

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
    fn preload_registers_the_output_preloads_without_includes() {
        let asset_bag = AssetBag::new();

        asset_bag.preload(script_stylesheet());

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
    fn script_registers_only_the_script_include_from_a_script_stylesheet_bundle() {
        let asset_bag = AssetBag::new();

        asset_bag.script(script_stylesheet());

        assert_eq!(
            head_of(&asset_bag),
            concat!(
                "<link rel=\"preload\" href=\"https://fonts.example/font.woff2\" as=\"font\" crossorigin>",
                "<link rel=\"modulepreload\" href=\"/assets/chunk_ABC12345.js\">",
                "<script async src=\"/assets/app_ABC12345.js\" type=\"module\"></script>",
            )
        );
    }

    #[test]
    fn stylesheet_registers_only_the_stylesheet_include_from_a_script_stylesheet_bundle() {
        let asset_bag = AssetBag::new();

        asset_bag.stylesheet(script_stylesheet());

        assert_eq!(
            head_of(&asset_bag),
            concat!(
                "<link rel=\"preload\" href=\"https://fonts.example/font.woff2\" as=\"font\" crossorigin>",
                "<link rel=\"modulepreload\" href=\"/assets/chunk_ABC12345.js\">",
                "<link rel=\"stylesheet\" href=\"/assets/app_ABC12345.css\">",
            )
        );
    }

    #[test]
    fn add_registers_a_script_only_bundle() {
        let asset_bag = AssetBag::new();

        asset_bag.add(script_only());

        assert_eq!(
            head_of(&asset_bag),
            concat!(
                "<link rel=\"preload\" href=\"https://fonts.example/font.woff2\" as=\"font\" crossorigin>",
                "<link rel=\"modulepreload\" href=\"/assets/chunk_ABC12345.js\">",
                "<script async src=\"/assets/solo_ABC12345.js\" type=\"module\"></script>",
            )
        );
    }

    #[test]
    fn preload_registers_a_script_only_bundle_output_preload() {
        let asset_bag = AssetBag::new();

        asset_bag.preload(script_only());

        assert_eq!(
            head_of(&asset_bag),
            concat!(
                "<link rel=\"preload\" href=\"https://fonts.example/font.woff2\" as=\"font\" crossorigin>",
                "<link rel=\"modulepreload\" href=\"/assets/chunk_ABC12345.js\">",
                "<link rel=\"modulepreload\" href=\"/assets/solo_ABC12345.js\">",
            )
        );
    }

    #[test]
    fn script_registers_a_script_only_bundle() {
        let asset_bag = AssetBag::new();

        asset_bag.script(script_only());

        assert_eq!(
            head_of(&asset_bag),
            concat!(
                "<link rel=\"preload\" href=\"https://fonts.example/font.woff2\" as=\"font\" crossorigin>",
                "<link rel=\"modulepreload\" href=\"/assets/chunk_ABC12345.js\">",
                "<script async src=\"/assets/solo_ABC12345.js\" type=\"module\"></script>",
            )
        );
    }

    #[test]
    fn add_registers_a_stylesheet_only_bundle() {
        let asset_bag = AssetBag::new();

        asset_bag.add(stylesheet_only());

        assert_eq!(
            head_of(&asset_bag),
            concat!(
                "<link rel=\"preload\" href=\"https://fonts.example/font.woff2\" as=\"font\" crossorigin>",
                "<link rel=\"modulepreload\" href=\"/assets/chunk_ABC12345.js\">",
                "<link rel=\"stylesheet\" href=\"/assets/theme_ABC12345.css\">",
            )
        );
    }

    #[test]
    fn preload_registers_a_stylesheet_only_bundle_output_preload() {
        let asset_bag = AssetBag::new();

        asset_bag.preload(stylesheet_only());

        assert_eq!(
            head_of(&asset_bag),
            concat!(
                "<link rel=\"preload\" href=\"https://fonts.example/font.woff2\" as=\"font\" crossorigin>",
                "<link rel=\"modulepreload\" href=\"/assets/chunk_ABC12345.js\">",
                "<link rel=\"preload\" href=\"/assets/theme_ABC12345.css\" as=\"style\">",
            )
        );
    }

    #[test]
    fn stylesheet_registers_a_stylesheet_only_bundle() {
        let asset_bag = AssetBag::new();

        asset_bag.stylesheet(stylesheet_only());

        assert_eq!(
            head_of(&asset_bag),
            concat!(
                "<link rel=\"preload\" href=\"https://fonts.example/font.woff2\" as=\"font\" crossorigin>",
                "<link rel=\"modulepreload\" href=\"/assets/chunk_ABC12345.js\">",
                "<link rel=\"stylesheet\" href=\"/assets/theme_ABC12345.css\">",
            )
        );
    }

    #[test]
    fn image_resolves_an_image_output_to_its_hashed_url() {
        let asset_bag = AssetBag::new();

        assert_eq!(
            asset_bag.image(AssetResolution::new(
                Absent,
                ImageOutput::new(AssetHref::Local("assets/logo_ABC12345.png")),
                Absent,
            )),
            "/assets/logo_ABC12345.png"
        );
    }

    #[test]
    fn file_resolves_a_file_output_to_its_hashed_url() {
        let asset_bag = AssetBag::new();

        assert_eq!(
            asset_bag.file(AssetResolution::new(
                Absent,
                Absent,
                FileOutput::new(AssetHref::Local("assets/inter_ABC12345.woff2")),
            )),
            "/assets/inter_ABC12345.woff2"
        );
    }

    #[test]
    fn external_script_and_stylesheet_emit_external_assets_after_the_includes() {
        let asset_bag = AssetBag::new();

        asset_bag.add(script_stylesheet());
        asset_bag.external_script("https://challenges.example/api.js");
        asset_bag.external_stylesheet("https://fonts.example/inter.css");

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
