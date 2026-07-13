use crate::views::card_layout::card_layout;
use crate::views::card_layout_props::CardLayoutProps;
use crate::views::greeting_view_props::GreetingViewProps;

pub fn greeting_view(GreetingViewProps { greeting, home_url }: GreetingViewProps) -> String {
    card_layout(CardLayoutProps {
        body_text: greeting,
        home_url,
    })
}
