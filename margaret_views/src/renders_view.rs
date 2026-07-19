use maud::Markup;

pub trait RendersView {
    type Props;

    #[must_use]
    fn render(&self, props: Self::Props) -> Markup;

    #[must_use]
    fn render_to_string(&self, props: Self::Props) -> String {
        self.render(props).into_string()
    }
}

#[cfg(test)]
mod tests {
    use maud::Markup;
    use maud::html;

    use super::RendersView;

    struct Greeting;

    impl RendersView for Greeting {
        type Props = String;

        fn render(&self, name: Self::Props) -> Markup {
            html! { p { "hi " (name) } }
        }
    }

    #[test]
    fn renders_a_view_directly_to_a_string() {
        assert_eq!(Greeting.render_to_string("ada".to_string()), "<p>hi ada</p>");
    }
}
