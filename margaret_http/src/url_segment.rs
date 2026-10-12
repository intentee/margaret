use crate::url_parameter::UrlParameter;

pub enum UrlSegment {
    CatchAllParameter {
        parameter: UrlParameter,
        prefix: &'static str,
    },
    Literal {
        decoded: &'static str,
        encoded: &'static str,
    },
    Parameter {
        parameter: UrlParameter,
        prefix: &'static str,
    },
}
