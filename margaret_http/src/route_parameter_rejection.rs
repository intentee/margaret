pub enum RouteParameterRejection {
    NotFound,
    SystemError(anyhow::Error),
}
