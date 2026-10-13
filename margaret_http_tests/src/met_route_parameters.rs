use std::future::Ready;
use std::future::ready;

use margaret_http::requirement::Requirement;

pub fn met_route_parameters<const COUNT: usize>(
    parameters: [String; COUNT],
) -> Ready<Requirement<[String; COUNT]>> {
    ready(Requirement::Met(parameters))
}
