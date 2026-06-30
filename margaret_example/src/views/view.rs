use margaret_http::http_interceptable::HttpInterceptable;

pub trait View: HttpInterceptable {
    fn body(&self) -> String;
}
