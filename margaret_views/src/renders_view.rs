use maud::Markup;

pub trait RendersView {
    type Props<'props>;

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    fn render(&self, props: Self::Props<'_>) -> anyhow::Result<Markup>;

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    fn render_to_string(&self, props: Self::Props<'_>) -> anyhow::Result<String> {
        Ok(self.render(props)?.into_string())
    }
}

#[cfg(test)]
mod tests {
    use maud::Markup;
    use maud::html;

    use super::RendersView;

    struct Greeting;

    struct Link;

    struct FailingView;

    struct LinkProps<'href> {
        href: &'href str,
    }

    impl RendersView for Greeting {
        type Props<'props> = String;

        fn render(&self, name: Self::Props<'_>) -> anyhow::Result<Markup> {
            Ok(html! { p { "hi " (name) } })
        }
    }

    impl RendersView for Link {
        type Props<'props> = LinkProps<'props>;

        fn render(&self, LinkProps { href }: Self::Props<'_>) -> anyhow::Result<Markup> {
            Ok(html! { a href=(href) { "go" } })
        }
    }

    impl RendersView for FailingView {
        type Props<'props> = ();

        fn render(&self, (): Self::Props<'_>) -> anyhow::Result<Markup> {
            Err(anyhow::anyhow!("template dependency unavailable"))
        }
    }

    #[test]
    fn renders_a_view_directly_to_a_string() {
        assert_eq!(
            Greeting
                .render_to_string("ada".to_string())
                .expect("the view renders"),
            "<p>hi ada</p>"
        );
    }

    #[test]
    fn renders_a_view_whose_props_borrow_a_reference() {
        let href = String::from("/home");

        assert_eq!(
            Link.render(LinkProps { href: &href })
                .expect("the view renders")
                .into_string(),
            "<a href=\"/home\">go</a>"
        );
    }

    #[test]
    fn propagates_a_view_rendering_failure() {
        let error = FailingView
            .render_to_string(())
            .expect_err("the view failure is propagated");

        assert_eq!(error.to_string(), "template dependency unavailable");
    }
}
