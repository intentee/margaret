use maud::Markup;
use maud::Render;
use maud::html;

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ExternalAsset {
    Script(String),
    Stylesheet(String),
}

impl Render for ExternalAsset {
    fn render(&self) -> Markup {
        match self {
            ExternalAsset::Script(url) => html! {
                script async defer src=(url) {}
            },
            ExternalAsset::Stylesheet(url) => html! {
                link rel="stylesheet" href=(url);
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use maud::Render;

    use super::ExternalAsset;

    #[test]
    fn renders_an_async_defer_script_tag() {
        assert_eq!(
            ExternalAsset::Script("https://challenges.example/api.js".to_string())
                .render()
                .into_string(),
            "<script async defer src=\"https://challenges.example/api.js\"></script>"
        );
    }

    #[test]
    fn renders_a_stylesheet_link_tag() {
        assert_eq!(
            ExternalAsset::Stylesheet("https://fonts.example/inter.css".to_string())
                .render()
                .into_string(),
            "<link rel=\"stylesheet\" href=\"https://fonts.example/inter.css\">"
        );
    }

    #[test]
    fn escapes_a_url_inside_the_attribute() {
        assert_eq!(
            ExternalAsset::Script("https://example/s.js?a=1&b=\"x\"".to_string())
                .render()
                .into_string(),
            "<script async defer src=\"https://example/s.js?a=1&amp;b=&quot;x&quot;\"></script>"
        );
    }
}
