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
    fn renders_an_external_script_async_and_deferred_so_it_never_blocks_parsing() {
        assert_eq!(
            ExternalAsset::Script("https://challenges.example/api.js".to_string())
                .render()
                .into_string(),
            "<script async defer src=\"https://challenges.example/api.js\"></script>"
        );
    }

    #[test]
    fn renders_an_external_stylesheet_link_to_its_url_as_given_without_a_root_prefix() {
        assert_eq!(
            ExternalAsset::Stylesheet("https://fonts.example/inter.css".to_string())
                .render()
                .into_string(),
            "<link rel=\"stylesheet\" href=\"https://fonts.example/inter.css\">"
        );
    }

    #[test]
    fn escapes_an_external_url_inside_the_attribute_so_it_cannot_break_out_of_it() {
        assert_eq!(
            ExternalAsset::Script("https://example/s.js?a=1&b=\"x\"".to_string())
                .render()
                .into_string(),
            "<script async defer src=\"https://example/s.js?a=1&amp;b=&quot;x&quot;\"></script>"
        );
    }
}
