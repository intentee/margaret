use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::route_parameters::book_number::BookNumber;
use crate::route_parameters::shelf_label::ShelfLabel;

#[singleton]
#[responds_to_http(
    method = RouteMethod::Get,
    name = "get_book",
    path = "/books/{shelf}/{number}/{title}",
    server = "public"
)]
pub struct GetBook;

impl GetBook {
    /// # Errors
    ///
    /// Never returns an error; every routed book is described.
    #[process]
    pub fn respond(
        &self,
        #[route_parameter(from = "shelf")] ShelfLabel(shelf): ShelfLabel,
        #[route_parameter(from = "number")] BookNumber(number): BookNumber,
        #[route_parameter(from = "title")] title: String,
    ) -> anyhow::Result<Response> {
        Ok(Response::text(200, format!("{shelf} #{number}: {title}")))
    }
}
