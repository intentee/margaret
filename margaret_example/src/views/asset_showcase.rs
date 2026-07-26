use margaret::framework::asset_bag::asset_bag::AssetBag;
use margaret::framework::macros::singleton;
use margaret::framework::views::maud::Markup;
use margaret::framework::views::maud::html;
use margaret::framework::views::renders_view::RendersView;

use crate::margaret::asset_bag::asset;

pub struct AssetShowcaseProps {
    pub asset_bag: AssetBag,
}

#[singleton]
pub struct AssetShowcase;

impl RendersView for AssetShowcase {
    type Props<'props> = AssetShowcaseProps;

    fn render(&self, AssetShowcaseProps { asset_bag }: Self::Props<'_>) -> anyhow::Result<Markup> {
        asset_bag.add(asset!("resources/ts/app.ts"));

        let logo = asset_bag.image(asset!("resources/media/logo.png"));
        let favicon = asset_bag.image(asset!("resources/media/favicon.svg"));
        let font = asset_bag.file(asset!("resources/fonts/inter.woff2"));

        Ok(html! {
            main {
                img src=(logo) alt="logo";
                img src=(favicon) alt="favicon";
                link rel="preload" href=(font) as="font" crossorigin;
                p { "Assets demo" }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use margaret::framework::asset_bag::asset_bag::AssetBag;
    use margaret::framework::views::maud::Render;
    use margaret::framework::views::renders_view::RendersView;

    use super::AssetShowcase;
    use super::AssetShowcaseProps;

    #[test]
    fn renders_static_assets_and_registers_the_entry_point() {
        let asset_bag = AssetBag::new();

        let markup = AssetShowcase
            .render(AssetShowcaseProps {
                asset_bag: asset_bag.clone(),
            })
            .expect("the asset showcase renders")
            .into_string();

        assert_eq!(
            markup,
            concat!(
                "<main>",
                "<img src=\"/assets/logo_I9J0K1L2.png\" alt=\"logo\">",
                "<img src=\"/assets/favicon_M1N2O3P4.svg\" alt=\"favicon\">",
                "<link rel=\"preload\" href=\"/assets/inter_U9V0W1X2.woff2\" as=\"font\" crossorigin>",
                "<p>Assets demo</p>",
                "</main>",
            )
        );
        assert!(
            asset_bag.head().render().into_string().contains(
                "<script async src=\"/assets/app_A1B2C3D4.js\" type=\"module\"></script>"
            )
        );
    }
}
