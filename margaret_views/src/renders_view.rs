use anyhow::Result;
use maud::Markup;

pub trait RendersView {
    type Props<'props>;

    fn render(&self, props: Self::Props<'_>) -> Result<Markup>;

    fn render_to_string(&self, props: Self::Props<'_>) -> Result<String> {
        Ok(self.render(props)?.into_string())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use anyhow::anyhow;
    use maud::Markup;
    use maud::html;

    use super::RendersView;

    struct Greeting;

    struct Link;

    struct Failing;

    struct LinkProps<'href> {
        href: &'href str,
    }

    impl RendersView for Greeting {
        type Props<'props> = String;

        fn render(&self, name: Self::Props<'_>) -> Result<Markup> {
            Ok(html! { p { "hi " (name) } })
        }
    }

    impl RendersView for Link {
        type Props<'props> = LinkProps<'props>;

        fn render(&self, LinkProps { href }: Self::Props<'_>) -> Result<Markup> {
            Ok(html! { a href=(href) { "go" } })
        }
    }

    impl RendersView for Failing {
        type Props<'props> = ();

        fn render(&self, (): Self::Props<'_>) -> Result<Markup> {
            Err(anyhow!("the view cannot render"))
        }
    }

    #[test]
    fn renders_a_view_directly_to_a_string() {
        assert_eq!(
            Greeting
                .render_to_string("ada".to_string())
                .expect("the greeting renders"),
            "<p>hi ada</p>"
        );
    }

    #[test]
    fn renders_a_view_whose_props_borrow_a_reference() {
        let href = String::from("/home");

        assert_eq!(
            Link.render(LinkProps { href: &href })
                .expect("the link renders")
                .into_string(),
            "<a href=\"/home\">go</a>"
        );
    }

    #[test]
    fn propagates_a_render_failure_through_render_to_string() {
        assert!(Failing.render_to_string(()).is_err());
    }
}
