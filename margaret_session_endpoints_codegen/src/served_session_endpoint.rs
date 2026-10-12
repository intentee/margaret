use margaret_serve_input_codegen::route_url_input::RouteUrlInput;

pub(crate) enum ServedSessionEndpoint {
    Refresh,
    SignOut { landing: RouteUrlInput },
}
