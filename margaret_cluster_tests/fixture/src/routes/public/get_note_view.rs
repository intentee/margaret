use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;
use margaret::framework::views::renders_view::RendersView;

use crate::margaret::views::Views;
use crate::models::note::Note;
use crate::views::note_view::NoteViewProps;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/notes/{note}/view", server = "public")]
pub struct GetNoteView;

impl GetNoteView {
    /// # Errors
    ///
    /// Returns an error when the note view cannot be rendered.
    #[process]
    pub fn respond(
        &self,
        views: &Views,
        #[route_parameter(from = "note")] note: Note,
    ) -> anyhow::Result<Response> {
        Ok(Response::html(
            200,
            views.note_view.render(NoteViewProps { note: &note })?,
        ))
    }
}
