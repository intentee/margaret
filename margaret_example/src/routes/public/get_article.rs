use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::models::article::Article;
use crate::models::author::Author;

#[singleton]
#[responds_to_http(
    method = RouteMethod::Get,
    name = "get_article",
    path = "/articles/{article}",
    server = "public"
)]
pub struct GetArticle;

impl GetArticle {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(
        &self,
        #[route_parameter(from = "article")] Article {
            title,
            body,
            price,
            reading_minutes,
            created_at,
            author,
            ..
        }: Article,
    ) -> anyhow::Result<Response> {
        let Author {
            name: author_name,
            reputation,
            ..
        } = author;

        Ok(Response::text(
            200,
            format!(
                "\"{title}\" by {author_name} (reputation {reputation}), posted at {created_at}, {price} for a {reading_minutes} minute read: {body}"
            ),
        ))
    }
}
