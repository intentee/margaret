use std::collections::HashSet;
use std::sync::Arc;

use margaret_cookie_jar::cookie_jar::CookieJar;

use crate::forward_targets::ForwardTargets;
use crate::handler::Handler;
use crate::request::Request;
use crate::response::Response;
use crate::response_continuation::ResponseContinuation;

pub(crate) async fn respond_recursively(
    forward_targets: &Arc<ForwardTargets>,
    request: Request,
    cookie_jar: &CookieJar,
    first: Arc<dyn Handler>,
) -> Response {
    let mut visited: HashSet<&'static str> = HashSet::new();
    let mut request = request;
    let mut outcome = first.handle(&request, cookie_jar).await;

    loop {
        match outcome {
            ResponseContinuation::Done(response) => return response,
            ResponseContinuation::Forward(forward) => {
                let name = forward.name();

                if !visited.insert(name) {
                    eprintln!("margaret_http: forward cycle re-entered the responder `{name}`");

                    return Response::text(500, "Internal Server Error");
                }

                let Some(target) = forward_targets.resolve(name) else {
                    eprintln!(
                        "margaret_http: no forward target is registered for `{name}` on this server"
                    );

                    return Response::text(500, "Internal Server Error");
                };

                request = request.with_path_params(forward.into_path_params());
                outcome = target.handle(&request, cookie_jar).await;
            }
            ResponseContinuation::Redirect(redirect) => return redirect.into_response(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use async_trait::async_trait;
    use cookie::Cookie;
    use http::HeaderMap;
    use http::Method;
    use http_body_util::BodyExt;

    use margaret_cookie_jar::cookie_jar::CookieJar;

    use super::respond_recursively;
    use crate::forward::Forward;
    use crate::forward_targets::ForwardTargets;
    use crate::handler::Handler;
    use crate::named_handler::NamedHandler;
    use crate::redirect::Redirect;
    use crate::request::Request;
    use crate::response::Response;
    use crate::response_continuation::ResponseContinuation;

    struct ForwardToTarget;

    #[async_trait]
    impl Handler for ForwardToTarget {
        async fn handle(
            &self,
            _request: &Request,
            _cookie_jar: &CookieJar,
        ) -> ResponseContinuation {
            ResponseContinuation::from(Forward::new("target", HashMap::new()))
        }
    }

    struct ForwardToSelf;

    #[async_trait]
    impl Handler for ForwardToSelf {
        async fn handle(
            &self,
            _request: &Request,
            _cookie_jar: &CookieJar,
        ) -> ResponseContinuation {
            ResponseContinuation::from(Forward::new("origin", HashMap::new()))
        }
    }

    struct ForwardToUnknown;

    #[async_trait]
    impl Handler for ForwardToUnknown {
        async fn handle(
            &self,
            _request: &Request,
            _cookie_jar: &CookieJar,
        ) -> ResponseContinuation {
            ResponseContinuation::from(Forward::new("unknown", HashMap::new()))
        }
    }

    struct Target;

    #[async_trait]
    impl Handler for Target {
        async fn handle(
            &self,
            _request: &Request,
            _cookie_jar: &CookieJar,
        ) -> ResponseContinuation {
            ResponseContinuation::Done(Response::text(222, "target"))
        }
    }

    struct ForwardToArticle;

    #[async_trait]
    impl Handler for ForwardToArticle {
        async fn handle(
            &self,
            _request: &Request,
            _cookie_jar: &CookieJar,
        ) -> ResponseContinuation {
            ResponseContinuation::from(Forward::new(
                "article",
                HashMap::from([("article".to_string(), "7".to_string())]),
            ))
        }
    }

    struct EchoArticle;

    #[async_trait]
    impl Handler for EchoArticle {
        async fn handle(&self, request: &Request, _cookie_jar: &CookieJar) -> ResponseContinuation {
            ResponseContinuation::Done(Response::text(
                200,
                request
                    .path_param("article")
                    .expect("the forward supplies the article path parameter")
                    .to_string(),
            ))
        }
    }

    struct RedirectingResponder;

    #[async_trait]
    impl Handler for RedirectingResponder {
        async fn handle(
            &self,
            _request: &Request,
            _cookie_jar: &CookieJar,
        ) -> ResponseContinuation {
            ResponseContinuation::from(Redirect::see_other("http://localhost/greeting".to_string()))
        }
    }

    struct StagesOriginCookie;

    #[async_trait]
    impl Handler for StagesOriginCookie {
        async fn handle(&self, _request: &Request, cookie_jar: &CookieJar) -> ResponseContinuation {
            cookie_jar
                .add(Cookie::build(("origin", "1")).build())
                .expect("the origin cookie is staged");

            ResponseContinuation::from(Forward::new("target", HashMap::new()))
        }
    }

    struct StagesTargetCookie;

    #[async_trait]
    impl Handler for StagesTargetCookie {
        async fn handle(&self, _request: &Request, cookie_jar: &CookieJar) -> ResponseContinuation {
            cookie_jar
                .add(Cookie::build(("target", "1")).build())
                .expect("the target cookie is staged");

            ResponseContinuation::Done(Response::text(200, "done"))
        }
    }

    fn forward_targets_with(targets: Vec<NamedHandler>) -> Arc<ForwardTargets> {
        Arc::new(ForwardTargets::new(targets))
    }

    async fn status_of(forward_targets: Arc<ForwardTargets>, first: Arc<dyn Handler>) -> u16 {
        respond_recursively(
            &forward_targets,
            Request::new(Method::GET, "/".to_string()),
            &CookieJar::from_headers(&HeaderMap::new()).expect("an empty cookie jar is built"),
            first,
        )
        .await
        .into_http()
        .status()
        .as_u16()
    }

    #[tokio::test]
    async fn forwards_to_the_target_responder() {
        let forward_targets =
            forward_targets_with(vec![NamedHandler::new("target", Arc::new(Target))]);

        assert_eq!(
            status_of(forward_targets, Arc::new(ForwardToTarget)).await,
            222
        );
    }

    #[tokio::test]
    async fn returns_a_server_error_on_a_forward_cycle() {
        let forward_targets =
            forward_targets_with(vec![NamedHandler::new("origin", Arc::new(ForwardToSelf))]);

        assert_eq!(
            status_of(forward_targets, Arc::new(ForwardToSelf)).await,
            500
        );
    }

    #[tokio::test]
    async fn returns_a_server_error_when_forwarding_to_an_unregistered_name() {
        assert_eq!(
            status_of(forward_targets_with(Vec::new()), Arc::new(ForwardToUnknown)).await,
            500
        );
    }

    #[tokio::test]
    async fn returns_a_redirect_response() {
        let response = respond_recursively(
            &forward_targets_with(Vec::new()),
            Request::new(Method::GET, "/".to_string()),
            &CookieJar::from_headers(&HeaderMap::new()).expect("an empty cookie jar is built"),
            Arc::new(RedirectingResponder),
        )
        .await
        .into_http();

        assert_eq!(response.status().as_u16(), 303);
        assert_eq!(
            response
                .headers()
                .get("location")
                .expect("the location header is present"),
            "http://localhost/greeting"
        );
    }

    #[tokio::test]
    async fn installs_forwarded_path_parameters_for_the_target() {
        let forward_targets =
            forward_targets_with(vec![NamedHandler::new("article", Arc::new(EchoArticle))]);
        let response = respond_recursively(
            &forward_targets,
            Request::new(Method::GET, "/".to_string()),
            &CookieJar::from_headers(&HeaderMap::new()).expect("an empty cookie jar is built"),
            Arc::new(ForwardToArticle),
        )
        .await
        .into_http();

        assert_eq!(response.status().as_u16(), 200);
        assert_eq!(
            response
                .into_body()
                .collect()
                .await
                .expect("the response body collects")
                .to_bytes(),
            "7"
        );
    }

    #[tokio::test]
    async fn shares_one_cookie_jar_across_a_forward() {
        let forward_targets = forward_targets_with(vec![NamedHandler::new(
            "target",
            Arc::new(StagesTargetCookie),
        )]);
        let cookie_jar =
            CookieJar::from_headers(&HeaderMap::new()).expect("an empty cookie jar is built");

        respond_recursively(
            &forward_targets,
            Request::new(Method::GET, "/".to_string()),
            &cookie_jar,
            Arc::new(StagesOriginCookie),
        )
        .await;

        assert_eq!(
            cookie_jar.into_set_cookie_values(),
            vec!["origin=1".to_owned(), "target=1".to_owned()]
        );
    }
}
