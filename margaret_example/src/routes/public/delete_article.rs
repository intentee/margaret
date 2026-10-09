use std::sync::Arc;

use margaret::framework::active_record::deletion::Deletion;
use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::models::article::Article;

#[singleton]
#[responds_to_http(method = RouteMethod::Delete, path = "/articles/{article}", server = "public")]
pub struct DeleteArticle {
    database: Arc<Database>,
}

impl DeleteArticle {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(database: Arc<Database>) -> anyhow::Result<Self> {
        Ok(Self { database })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub async fn respond(
        &self,
        #[route_parameter(from = "article")] article: Article,
    ) -> anyhow::Result<Response> {
        Ok(match article.delete(self.database.as_ref()).await? {
            Deletion::Deleted => Response::text(200, format!("deleted \"{}\"", article.title)),
            Deletion::Missing => Response::not_found(),
        })
    }
}
