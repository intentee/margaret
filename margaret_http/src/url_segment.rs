use crate::url_parameter::UrlParameter;

pub enum UrlSegment {
    Literal(&'static str),
    Parameter(UrlParameter),
}
