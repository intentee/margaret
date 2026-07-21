use maud::Markup;

pub trait RendersView {
    type Props<'props>;

    #[must_use]
    fn render(&self, props: Self::Props<'_>) -> Markup;

    #[must_use]
    fn render_to_string(&self, props: Self::Props<'_>) -> String {
        self.render(props).into_string()
    }
}

#[cfg(test)]
mod tests {
    use maud::Markup;
    use maud::html;

    use super::RendersView;

    struct Greeting;

    struct Link;

    struct LinkProps<'href> {
        href: &'href str,
    }

    impl RendersView for Greeting {
        type Props<'props> = String;

        fn render(&self, name: Self::Props<'_>) -> Markup {
            html! { p { "hi " (name) } }
        }
    }

    impl RendersView for Link {
        type Props<'props> = LinkProps<'props>;

        fn render(&self, LinkProps { href }: Self::Props<'_>) -> Markup {
            html! { a href=(href) { "go" } }
        }
    }

    #[test]
    fn renders_a_view_directly_to_a_string() {
        assert_eq!(
            Greeting.render_to_string("ada".to_string()),
            "<p>hi ada</p>"
        );
    }

    #[test]
    fn renders_a_view_whose_props_borrow_a_reference() {
        let href = String::from("/home");

        assert_eq!(
            Link.render(LinkProps { href: &href }).into_string(),
            "<a href=\"/home\">go</a>"
        );
    }
}
