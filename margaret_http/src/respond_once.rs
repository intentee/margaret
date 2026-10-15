use crate::one_shot_handler::OneShotHandler;
use crate::request::Request;
use crate::respond_to_outcome::respond_to_outcome;
use crate::response::Response;

pub(crate) async fn respond_once(request: &Request, handler: Box<dyn OneShotHandler>) -> Response {
    respond_to_outcome(handler.handle(request).await)
}
