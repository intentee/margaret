pub enum RouteParameterBindingOutcome<Model> {
    Bound(Model),
    NotFound,
}
