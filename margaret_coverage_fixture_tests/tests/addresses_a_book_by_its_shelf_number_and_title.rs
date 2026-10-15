use margaret_coverage_fixture::margaret::framework::http::route_addressing::RouteAddressing;
use margaret_coverage_fixture::margaret::framework::url_path::path_parameter_value::PathParameterValue;
use margaret_coverage_fixture::margaret::routes::Routes;
use margaret_coverage_fixture::margaret::routes::props::GetBookRouteProps;

#[test]
fn addresses_a_book_by_its_shelf_number_and_title() {
    assert_eq!(
        Routes::from_origins("http://library.test")
            .expect("the routes of the library build")
            .public
            .get_book
            .address(GetBookRouteProps {
                number: PathParameterValue::new("42".to_string())
                    .expect("the number fills a segment"),
                shelf: PathParameterValue::new("fiction".to_string())
                    .expect("the shelf fills a segment"),
                title: PathParameterValue::new("Dune".to_string())
                    .expect("the title fills a segment"),
            })
            .map(|book| book.url()),
        RouteAddressing::Addressed("http://library.test/books/fiction/42/Dune".to_string())
    );
}
