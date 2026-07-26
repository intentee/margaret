#[derive(Debug, PartialEq)]
pub enum RouteParameterOutcome<Model> {
    Found(Model),
    NotFound,
}
