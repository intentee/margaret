use std::sync::Arc;

use margaret_macros::constructor;
use margaret_macros::singleton;
use margaret_views::maud::DOCTYPE;
use margaret_views::maud::Markup;
use margaret_views::maud::html;
use margaret_views::renders_view::RendersView;

use crate::views::asset_page_props::AssetPageProps;
use crate::views::asset_showcase::AssetShowcase;
use crate::views::asset_showcase_props::AssetShowcaseProps;

#[singleton]
pub struct AssetPage {
    showcase: Arc<AssetShowcase>,
}

impl AssetPage {
    #[constructor]
    #[must_use]
    pub fn create(showcase: Arc<AssetShowcase>) -> Self {
        Self { showcase }
    }
}

impl RendersView for AssetPage {
    type Props = AssetPageProps;

    fn render(&self, AssetPageProps { asset_bag }: AssetPageProps) -> Markup {
        let body = self.showcase.render(AssetShowcaseProps {
            asset_bag: asset_bag.clone(),
        });

        html! {
            (DOCTYPE)
            html {
                head {
                    (asset_bag.head())
                }
                body {
                    (body)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use margaret_asset_bag::asset_bag::AssetBag;
    use margaret_views::renders_view::RendersView;

    use super::AssetPage;
    use crate::views::asset_page_props::AssetPageProps;
    use crate::views::asset_showcase::AssetShowcase;

    #[test]
    fn renders_the_head_preloads_before_the_body_that_registers_them() {
        let page = AssetPage {
            showcase: Arc::new(AssetShowcase),
        };

        let markup = page
            .render(AssetPageProps {
                asset_bag: AssetBag::new(),
            })
            .into_string();

        let head_end = markup.find("</head>").expect("the head is closed");
        let logo = markup
            .find("logo_I9J0K1L2.png")
            .expect("the logo is rendered in the body");

        assert!(markup.contains("<link rel=\"modulepreload\" href=\"/assets/chunk_E5F6G7H8.js\">"));
        assert!(
            markup.contains(
                "<script async src=\"/assets/app_A1B2C3D4.js\" type=\"module\"></script>"
            )
        );
        assert!(logo > head_end);
    }
}
