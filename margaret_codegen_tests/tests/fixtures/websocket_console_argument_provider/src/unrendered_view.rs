use margaret_macros::renders_view;
use margaret_macros::singleton;
use margaret_views::maud::Markup;
use margaret_views::maud::html;
use margaret_views::renders_view::RendersView;

use crate::margaret::asset_bag::asset;

pub struct UnrenderedViewProps {
    pub label: String,
}

#[renders_view(name = "unrendered_view")]
#[singleton]
pub struct UnrenderedView;

impl RendersView for UnrenderedView {
    type Props<'props> = UnrenderedViewProps;

    fn render(&self, UnrenderedViewProps { label }: Self::Props<'_>) -> Markup {
        let _ = asset!("resources/ts/app.ts");

        html! { span { (label) } }
    }
}
