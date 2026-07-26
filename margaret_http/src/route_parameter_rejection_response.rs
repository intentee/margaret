use crate::respond_with_user_error::respond_with_user_error;
use crate::response::Response;
use crate::route_parameter_rejection::RouteParameterRejection;

fn combine_system_errors(errors: Vec<anyhow::Error>) -> Option<anyhow::Error> {
    errors
        .into_iter()
        .reduce(|accumulated, error| accumulated.context(format!("{error:#}")))
}

#[must_use]
pub fn route_parameter_rejection_response<const COUNT: usize>(
    rejections: [Option<RouteParameterRejection>; COUNT],
) -> Response {
    let mut system_errors: Vec<anyhow::Error> = Vec::new();

    for rejection in rejections {
        if let Some(RouteParameterRejection::SystemError(error)) = rejection {
            system_errors.push(error);
        }
    }

    match combine_system_errors(system_errors) {
        Some(error) => respond_with_user_error(error),
        None => Response::not_found(),
    }
}

#[cfg(test)]
mod tests {
    use super::combine_system_errors;
    use super::route_parameter_rejection_response;
    use crate::route_parameter_rejection::RouteParameterRejection;

    #[test]
    fn a_single_not_found_becomes_a_404() {
        assert_eq!(
            route_parameter_rejection_response([Some(RouteParameterRejection::NotFound)]).status(),
            404
        );
    }

    #[test]
    fn a_single_system_error_becomes_a_500() {
        assert_eq!(
            route_parameter_rejection_response([Some(RouteParameterRejection::SystemError(
                anyhow::anyhow!("the datastore is unavailable")
            ))])
            .status(),
            500
        );
    }

    #[test]
    fn a_system_error_takes_precedence_over_a_not_found() {
        assert_eq!(
            route_parameter_rejection_response([
                Some(RouteParameterRejection::NotFound),
                Some(RouteParameterRejection::SystemError(anyhow::anyhow!("down"))),
            ])
            .status(),
            500
        );
    }

    #[test]
    fn the_precedence_is_independent_of_position() {
        assert_eq!(
            route_parameter_rejection_response([
                Some(RouteParameterRejection::SystemError(anyhow::anyhow!("down"))),
                Some(RouteParameterRejection::NotFound),
            ])
            .status(),
            500
        );
    }

    #[test]
    fn only_not_founds_become_a_404() {
        assert_eq!(
            route_parameter_rejection_response([
                Some(RouteParameterRejection::NotFound),
                Some(RouteParameterRejection::NotFound),
            ])
            .status(),
            404
        );
    }

    #[test]
    fn combining_system_errors_preserves_every_error() {
        let combined = combine_system_errors(vec![
            anyhow::anyhow!("primary datastore is unavailable"),
            anyhow::anyhow!("replica datastore is unavailable"),
        ])
        .expect("two errors combine into one");

        let rendered = format!("{combined:#}");

        assert!(rendered.contains("primary datastore is unavailable"));
        assert!(rendered.contains("replica datastore is unavailable"));
    }

    #[test]
    fn combining_no_system_errors_yields_nothing() {
        assert!(combine_system_errors(Vec::new()).is_none());
    }
}
