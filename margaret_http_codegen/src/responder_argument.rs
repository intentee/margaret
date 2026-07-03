use proc_macro2::Ident;

use crate::responder_argument_binding::ResponderArgumentBinding;

pub(crate) struct ResponderArgument {
    pub(crate) holder: Ident,
    pub(crate) binding: ResponderArgumentBinding,
}
