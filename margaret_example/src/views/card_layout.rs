use margaret_macros::renders_view;
use margaret_macros::singleton;
use margaret_views::maud::Markup;
use margaret_views::maud::html;
use margaret_views::renders_view::RendersView;

pub struct CardLayoutProps {
    pub body: Markup,
    pub home_url: String,
}

#[renders_view(name = "card_layout")]
#[singleton]
pub struct CardLayout;

impl RendersView for CardLayout {
    type Props<'props> = CardLayoutProps;

    fn render(&self, CardLayoutProps { body, home_url }: Self::Props<'_>) -> Markup {
        html! {
            main { (body) }
            nav {
                a href=(home_url) { "home" }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_views::maud::html;
    use margaret_views::renders_view::RendersView;

    use super::CardLayout;
    use super::CardLayoutProps;

    #[test]
    fn wraps_the_body_and_links_home() {
        let markup = CardLayout.render(CardLayoutProps {
            body: html! { "hello" },
            home_url: "/greeting".to_string(),
        });

        assert_eq!(
            markup.into_string(),
            "<main>hello</main><nav><a href=\"/greeting\">home</a></nav>"
        );
    }
}
