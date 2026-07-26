use crate::reject_route_path_segment::reject_route_path_segment;
use crate::route_url_error::RouteUrlError;

pub fn reject_route_path_tail(parameter: &str, value: &str) -> Result<(), RouteUrlError> {
    for segment in value.split('/') {
        reject_route_path_segment(parameter, segment)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::reject_route_path_tail;
    use crate::route_url_error::RouteUrlError;

    #[test]
    fn accepts_a_multi_segment_asset_path() {
        assert!(reject_route_path_tail("asset_path", "css/app.css").is_ok());
    }

    #[test]
    fn rejects_a_parent_segment_within_the_tail() {
        assert!(matches!(
            reject_route_path_tail("asset_path", "css/../secret")
                .expect_err("a traversal sub-segment is rejected"),
            RouteUrlError::ReservedPathSegment { parameter, value }
                if parameter == "asset_path" && value == ".."
        ));
    }
}
