use crate::url_segment::UrlSegment;

pub trait RouteProps {
    const NAME: &'static str;

    type Segments: AsRef<[UrlSegment]> + IntoIterator<Item = UrlSegment>;

    fn into_segments(self) -> Self::Segments;
}
