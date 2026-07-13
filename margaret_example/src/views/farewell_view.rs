use crate::views::card_layout::card_layout;
use crate::views::card_layout_props::CardLayoutProps;
use crate::views::farewell_view_props::FarewellViewProps;

pub fn farewell_view(FarewellViewProps { home_url, name }: FarewellViewProps) -> String {
    card_layout(CardLayoutProps {
        body_text: format!("goodbye, {name}"),
        home_url,
    })
}
