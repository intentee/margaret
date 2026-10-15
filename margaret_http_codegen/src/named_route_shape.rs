use proc_macro2::Ident;

use crate::route_props_layout::RoutePropsLayout;

pub(crate) enum NamedRouteShape {
    ForwardableLiteral {
        field: Ident,
        literal: String,
        props: RoutePropsLayout,
        variant: Ident,
    },
    ForwardableParameterized {
        field: Ident,
        props: RoutePropsLayout,
        variant: Ident,
    },
    ReferenceLiteral {
        literal: String,
    },
    ReferenceParameterized {
        props: RoutePropsLayout,
    },
}
