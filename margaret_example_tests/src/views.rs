use std::sync::Arc;

use margaret_example::margaret::views::Views;
use margaret_example::views::card_layout::CardLayout;
use margaret_example::views::consent_view::ConsentView;
use margaret_example::views::farewell_view::FarewellView;
use margaret_example::views::greeting_view::GreetingView;

/// # Panics
///
/// Panics when a view cannot be constructed.
#[must_use]
pub fn views() -> Views {
    let card_layout = Arc::new(CardLayout);

    Views {
        card_layout: card_layout.clone(),
        consent_view: Arc::new(ConsentView::create(card_layout.clone())),
        farewell_view: Arc::new(FarewellView::create(card_layout.clone())),
        greeting_view: Arc::new(GreetingView::create(card_layout)),
    }
}
