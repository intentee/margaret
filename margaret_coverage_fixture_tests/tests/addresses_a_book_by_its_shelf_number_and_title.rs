use margaret_coverage_fixture::margaret::framework::http::route_addressing::RouteAddressing;
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
                number: "42".to_string(),
                shelf: "fiction".to_string(),
                title: "Dune".to_string(),
            })
            .map(|book| book.url()),
        RouteAddressing::Addressed("http://library.test/books/fiction/42/Dune".to_string())
    );
}
