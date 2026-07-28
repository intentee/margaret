use crate::url_parameter::UrlParameter;

pub enum UrlSegment {
    CatchAllParameter(UrlParameter),
    Literal(&'static str),
    SegmentParameter(UrlParameter),
}
