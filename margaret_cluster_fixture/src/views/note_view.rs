use margaret::framework::macros::constructor;
use margaret::framework::macros::renders_view;
use margaret::framework::macros::singleton;
use margaret::framework::views::maud::Markup;
use margaret::framework::views::maud::html;
use margaret::framework::views::renders_view::RendersView;

use crate::models::note::Note;

pub struct NoteViewProps<'note> {
    pub note: &'note Note,
}

#[renders_view(name = "note_view")]
#[singleton]
pub struct NoteView;

impl NoteView {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create() -> anyhow::Result<Self> {
        Ok(Self)
    }
}

impl RendersView for NoteView {
    type Props<'props> = NoteViewProps<'props>;

    fn render(&self, NoteViewProps { note }: Self::Props<'_>) -> anyhow::Result<Markup> {
        Ok(html! { article { (note.body) } })
    }
}
