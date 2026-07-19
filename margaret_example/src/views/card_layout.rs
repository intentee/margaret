use crate::views::card_layout_props::CardLayoutProps;

#[must_use]
pub fn card_layout(
    CardLayoutProps {
        body_text,
        home_url,
    }: CardLayoutProps,
) -> String {
    format!("<main>{body_text}</main><nav><a href=\"{home_url}\">home</a></nav>")
}
