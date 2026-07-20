use maud::Markup;
use maud::Render;
use maud::html;

use crate::asset_href::AssetHref;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Include {
    Script(AssetHref),
    Stylesheet(AssetHref),
}

impl Render for Include {
    fn render(&self) -> Markup {
        match self {
            Include::Script(href) => html! {
                script async src=(href.url()) type="module" {}
            },
            Include::Stylesheet(href) => html! {
                link rel="stylesheet" href=(href.url());
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use maud::Render;

    use super::Include;
    use crate::asset_href::AssetHref;

    #[test]
    fn renders_a_module_script_tag() {
        assert_eq!(
            Include::Script(AssetHref::Local("assets/app_ABC12345.js"))
                .render()
                .into_string(),
            "<script async src=\"/assets/app_ABC12345.js\" type=\"module\"></script>"
        );
    }

    #[test]
    fn renders_a_stylesheet_link_tag() {
        assert_eq!(
            Include::Stylesheet(AssetHref::Local("assets/app_ABC12345.css"))
                .render()
                .into_string(),
            "<link rel=\"stylesheet\" href=\"/assets/app_ABC12345.css\">"
        );
    }
}
