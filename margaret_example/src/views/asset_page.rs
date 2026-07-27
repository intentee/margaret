use std::sync::Arc;

use margaret::framework::asset_bag::asset_bag::AssetBag;
use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;
use margaret::framework::views::maud::DOCTYPE;
use margaret::framework::views::maud::Markup;
use margaret::framework::views::maud::html;
use margaret::framework::views::renders_view::RendersView;

use crate::views::asset_showcase::AssetShowcase;
use crate::views::asset_showcase::AssetShowcaseProps;

pub struct AssetPageProps {
    pub asset_bag: AssetBag,
}

#[singleton]
pub struct AssetPage {
    showcase: Arc<AssetShowcase>,
}

impl AssetPage {
    #[constructor]
    pub fn create(showcase: Arc<AssetShowcase>) -> anyhow::Result<Self> {
        Ok(Self { showcase })
    }
}

impl RendersView for AssetPage {
    type Props<'props> = AssetPageProps;

    fn render(&self, AssetPageProps { asset_bag }: Self::Props<'_>) -> anyhow::Result<Markup> {
        Ok({
            let body = self.showcase.render(AssetShowcaseProps {
                asset_bag: asset_bag.clone(),
            })?;

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
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use margaret::framework::asset_bag::asset_bag::AssetBag;
    use margaret::framework::views::renders_view::RendersView;

    use super::AssetPage;
    use super::AssetPageProps;
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
            .expect("the asset page renders")
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
