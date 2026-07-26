use crate::url_parameter::UrlParameter;

pub enum UrlSegment {
    CatchAll(UrlParameter),
    Literal(&'static str),
    Parameter(UrlParameter),
}
