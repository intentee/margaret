use proc_macro2::Ident;

use crate::props_segment::PropsSegment;

pub(crate) struct RoutePropsLayout {
    pub(crate) module: Ident,
    pub(crate) segments: Vec<PropsSegment>,
    pub(crate) type_ident: Ident,
}
