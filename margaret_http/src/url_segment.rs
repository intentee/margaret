use crate::url_parameter::UrlParameter;

pub enum UrlSegment {
    CatchAllParameter {
        parameter: UrlParameter,
        prefix: &'static str,
    },
    Literal(&'static str),
    Parameter {
        parameter: UrlParameter,
        prefix: &'static str,
        suffix: &'static str,
    },
}
