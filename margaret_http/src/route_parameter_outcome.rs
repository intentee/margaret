pub enum RouteParameterOutcome<Model> {
    Found(Model),
    NotFound,
}
