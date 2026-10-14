use crate::addressable_route::AddressableRoute;
use crate::forward::Forward;
use crate::get_route_reference::GetRouteReference;
use crate::redirect::Redirect;
use crate::route_addressing::RouteAddressing;
use crate::route_props::RouteProps;

pub struct ForwardableRoute<TTarget, TProps, TLocator> {
    locator: TLocator,
    target: fn(TProps) -> TTarget,
}

impl<TTarget, TProps, TLocator> ForwardableRoute<TTarget, TProps, TLocator> {
    #[must_use]
    pub fn new(locator: TLocator, target: fn(TProps) -> TTarget) -> Self {
        Self { locator, target }
    }

    #[must_use]
    pub fn forward_to(&self, props: TProps) -> Forward<TTarget> {
        Forward {
            cookies: Vec::new(),
            target: (self.target)(props),
        }
    }
}

impl<TTarget, TProps> ForwardableRoute<TTarget, TProps, GetRouteReference> {
    #[must_use]
    pub fn permanent_redirect(&self) -> Redirect {
        self.locator.permanent_redirect()
    }

    #[must_use]
    pub fn see_other(&self) -> Redirect {
        self.locator.see_other()
    }

    #[must_use]
    pub fn temporary_redirect(&self) -> Redirect {
        self.locator.temporary_redirect()
    }

    #[must_use]
    pub fn url(&self) -> String {
        self.locator.url()
    }
}

impl<TTarget, TProps: RouteProps>
    ForwardableRoute<TTarget, TProps, AddressableRoute<TProps, GetRouteReference>>
{
    pub fn address(&self, props: TProps) -> RouteAddressing<GetRouteReference> {
        self.locator.address(props)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use http::header::LOCATION;

    use super::ForwardableRoute;
    use crate::get_route_reference::GetRouteReference;
    use crate::redirect::Redirect;
    use crate::route_addressing::RouteAddressing;
    use crate::route_position::RoutePosition;
    use crate::route_props::RouteProps;
    use crate::server_paths::ServerPaths;
    use crate::url_parameter::UrlParameter;
    use crate::url_segment::UrlSegment;

    struct NoteProps {
        note: String,
    }

    impl RouteProps for NoteProps {
        const NAME: &'static str = "get_note";

        type Segments = [UrlSegment; 2];

        fn into_segments(self) -> Self::Segments {
            [
                UrlSegment::Literal {
                    decoded: "/notes/",
                    encoded: "/notes/",
                },
                UrlSegment::Parameter {
                    parameter: UrlParameter {
                        name: "note",
                        value: self.note,
                    },
                    prefix: "",
                },
            ]
        }
    }

    enum NotesTarget {
        Note(NoteProps),
    }

    fn literal_route() -> ForwardableRoute<NotesTarget, NoteProps, GetRouteReference> {
        ForwardableRoute::new(
            GetRouteReference::new("http://localhost/notes/latest".to_string()),
            NotesTarget::Note,
        )
    }

    fn location(redirect: Redirect) -> String {
        redirect
            .into_response()
            .header_value(&LOCATION)
            .expect("the redirect names its location")
            .to_string()
    }

    #[test]
    fn forwards_to_the_target_its_props_name() {
        let NotesTarget::Note(NoteProps { note }) = literal_route()
            .forward_to(NoteProps {
                note: "rust".to_string(),
            })
            .target;

        assert_eq!(note, "rust");
    }

    #[test]
    fn answers_the_url_of_a_literal_route() {
        assert_eq!(literal_route().url(), "http://localhost/notes/latest");
    }

    #[test]
    fn sees_other_to_a_literal_route() {
        assert_eq!(
            location(literal_route().see_other()),
            "http://localhost/notes/latest"
        );
    }

    #[test]
    fn temporarily_redirects_to_a_literal_route() {
        assert_eq!(
            location(literal_route().temporary_redirect()),
            "http://localhost/notes/latest"
        );
    }

    #[test]
    fn permanently_redirects_to_a_literal_route() {
        assert_eq!(
            location(literal_route().permanent_redirect()),
            "http://localhost/notes/latest"
        );
    }

    #[test]
    fn addresses_a_parameterized_route_with_its_props() {
        let paths = ServerPaths::build(["/notes/{note}"]).expect("the route paths build");
        let route = ForwardableRoute::new(
            paths.addressable(
                &Arc::from("http://localhost"),
                RoutePosition::new(0),
                GetRouteReference::new,
            ),
            NotesTarget::Note,
        );

        assert!(matches!(
            route.address(NoteProps {
                note: "rust".to_string(),
            }),
            RouteAddressing::Addressed(reference) if reference.url() == "http://localhost/notes/rust"
        ));
    }
}
