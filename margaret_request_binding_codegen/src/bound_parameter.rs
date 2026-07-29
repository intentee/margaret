use proc_macro2::Ident;

use crate::request_binding::RequestBinding;

pub struct BoundParameter {
    pub binding: RequestBinding,
    pub declared_by_reference: bool,
    pub holder: Ident,
}
