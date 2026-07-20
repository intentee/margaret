use proc_macro2::Ident;

use crate::request_binding::RequestBinding;

pub struct BoundParameter {
    pub binding: RequestBinding,
    pub holder: Ident,
}
