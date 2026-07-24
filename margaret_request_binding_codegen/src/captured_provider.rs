use proc_macro2::Ident;

use crate::captured_provider_kind::CapturedProviderKind;

pub struct CapturedProvider {
    pub kind: CapturedProviderKind,
    pub local: Ident,
}
