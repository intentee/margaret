use margaret_macros::singleton;
use margaret_views::maud::Markup;
use margaret_views::maud::html;
use margaret_views::renders_view::RendersView;

use crate::margaret::asset_bag::asset;
use crate::views::asset_showcase_props::AssetShowcaseProps;

#[singleton]
pub struct AssetShowcase;

impl RendersView for AssetShowcase {
    type Props = AssetShowcaseProps;

    fn render(&self, AssetShowcaseProps { asset_bag }: AssetShowcaseProps) -> Markup {
        asset_bag.add(asset!("resources/ts/app.ts"));

        let logo = asset_bag.image(asset!("resources/media/logo.png"));

        html! {
            main {
                img src=(logo) alt="logo";
                p { "Assets demo" }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_asset_bag::asset_bag::AssetBag;
    use margaret_views::maud::Render;
    use margaret_views::renders_view::RendersView;

    use super::AssetShowcase;
    use crate::views::asset_showcase_props::AssetShowcaseProps;

    #[test]
    fn renders_the_logo_and_registers_the_entry_point() {
        let asset_bag = AssetBag::new();

        let markup = AssetShowcase
            .render(AssetShowcaseProps {
                asset_bag: asset_bag.clone(),
            })
            .into_string();

        assert_eq!(
            markup,
            "<main><img src=\"/assets/logo_I9J0K1L2.png\" alt=\"logo\"><p>Assets demo</p></main>"
        );
        assert!(
            asset_bag.head().render().into_string().contains(
                "<script async src=\"/assets/app_A1B2C3D4.js\" type=\"module\"></script>"
            )
        );
    }
}
