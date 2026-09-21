use margaret::framework::macros::renders_view;
use margaret::framework::macros::singleton;
use margaret::framework::views::maud::Markup;
use margaret::framework::views::maud::html;
use margaret::framework::views::renders_view::RendersView;

use crate::margaret::asset_bag::asset;
use crate::unrendered_view_props::UnrenderedViewProps;

#[renders_view(name = "unrendered_view")]
#[singleton]
pub struct UnrenderedView;

impl RendersView for UnrenderedView {
    type Props<'props> = UnrenderedViewProps;

    fn render(&self, UnrenderedViewProps { label }: Self::Props<'_>) -> anyhow::Result<Markup> {
        Ok({
            let _ = asset!("resources/ts/app.ts");

            html! { span { (label) } }
        })
    }
}
