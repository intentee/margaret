#[derive(Debug, Eq, PartialEq)]
pub enum RouteAddressing<Route> {
    Addressed(Route),
    Misrouted { url: String },
}

impl<Route> RouteAddressing<Route> {
    pub fn map<Mapped>(self, map: impl FnOnce(Route) -> Mapped) -> RouteAddressing<Mapped> {
        match self {
            Self::Addressed(route) => RouteAddressing::Addressed(map(route)),
            Self::Misrouted { url } => RouteAddressing::Misrouted { url },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RouteAddressing;

    #[test]
    fn maps_the_route_of_an_addressed_outcome_while_keeping_it_addressed() {
        assert_eq!(
            RouteAddressing::Addressed(2).map(|doubled| doubled * 2),
            RouteAddressing::Addressed(4)
        );
    }

    #[test]
    fn keeps_the_misrouted_url_through_a_map_so_callers_can_report_it() {
        assert_eq!(
            RouteAddressing::<u8>::Misrouted {
                url: "http://localhost/articles/new".to_string(),
            }
            .map(u16::from),
            RouteAddressing::Misrouted {
                url: "http://localhost/articles/new".to_string(),
            }
        );
    }
}
