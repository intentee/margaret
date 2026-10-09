use crate::unaddressable_parameter::UnaddressableParameter;

#[derive(Debug, Eq, PartialEq)]
pub enum RouteAddressing<Route> {
    Addressed(Route),
    Unaddressable(UnaddressableParameter),
}

impl<Route> RouteAddressing<Route> {
    pub fn map<Mapped>(self, map: impl FnOnce(Route) -> Mapped) -> RouteAddressing<Mapped> {
        match self {
            Self::Addressed(route) => RouteAddressing::Addressed(map(route)),
            Self::Unaddressable(parameter) => RouteAddressing::Unaddressable(parameter),
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_url_path::path_segment_rejection::PathSegmentRejection;

    use super::RouteAddressing;
    use crate::unaddressable_parameter::UnaddressableParameter;

    #[test]
    fn maps_an_addressed_route() {
        assert_eq!(
            RouteAddressing::Addressed(2).map(|doubled| doubled * 2),
            RouteAddressing::Addressed(4)
        );
    }

    #[test]
    fn keeps_the_unaddressable_parameter_through_a_map() {
        let parameter = UnaddressableParameter {
            name: "article",
            rejection: PathSegmentRejection::DotSegment,
        };

        assert_eq!(
            RouteAddressing::<u8>::Unaddressable(parameter).map(u16::from),
            RouteAddressing::Unaddressable(parameter)
        );
    }
}
