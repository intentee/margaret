use maud::Markup;
use maud::Render;
use maud::html;

use crate::asset_href::AssetHref;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Preload {
    Fetch(AssetHref),
    Font(AssetHref),
    Image(AssetHref),
    Module(AssetHref),
    Style(AssetHref),
}

impl Render for Preload {
    fn render(&self) -> Markup {
        match self {
            Preload::Fetch(href) => html! {
                link rel="preload" href=(href.url()) as="fetch" crossorigin;
            },
            Preload::Font(href) => html! {
                link rel="preload" href=(href.url()) as="font" crossorigin;
            },
            Preload::Image(href) => html! {
                link rel="preload" href=(href.url()) as="image";
            },
            Preload::Module(href) => html! {
                link rel="modulepreload" href=(href.url());
            },
            Preload::Style(href) => html! {
                link rel="preload" href=(href.url()) as="style";
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use maud::Render;

    use super::Preload;
    use crate::asset_href::AssetHref;

    fn rendered(preload: Preload) -> String {
        preload.render().into_string()
    }

    #[test]
    fn renders_a_fetch_preload_with_crossorigin() {
        assert_eq!(
            rendered(Preload::Fetch(AssetHref::Local("assets/data_ABC12345.bin"))),
            "<link rel=\"preload\" href=\"/assets/data_ABC12345.bin\" as=\"fetch\" crossorigin>"
        );
    }

    #[test]
    fn renders_a_font_preload_with_crossorigin() {
        assert_eq!(
            rendered(Preload::Font(AssetHref::Absolute(
                "https://fonts.example/font.woff2"
            ))),
            "<link rel=\"preload\" href=\"https://fonts.example/font.woff2\" as=\"font\" crossorigin>"
        );
    }

    #[test]
    fn renders_an_image_preload() {
        assert_eq!(
            rendered(Preload::Image(AssetHref::Local("assets/logo_ABC12345.png"))),
            "<link rel=\"preload\" href=\"/assets/logo_ABC12345.png\" as=\"image\">"
        );
    }

    #[test]
    fn renders_a_module_preload() {
        assert_eq!(
            rendered(Preload::Module(AssetHref::Local("assets/chunk_ABC12345.js"))),
            "<link rel=\"modulepreload\" href=\"/assets/chunk_ABC12345.js\">"
        );
    }

    #[test]
    fn renders_a_style_preload() {
        assert_eq!(
            rendered(Preload::Style(AssetHref::Local("assets/page_ABC12345.css"))),
            "<link rel=\"preload\" href=\"/assets/page_ABC12345.css\" as=\"style\">"
        );
    }
}
