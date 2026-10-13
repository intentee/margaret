use margaret_container::container_bindings::ContainerBindings;
use margaret_tag_codegen::bearer_token_addressee::BearerTokenAddressee;

#[derive(Clone, Copy)]
pub(crate) enum BearerTokenMarker<'marker> {
    Scanned {
        addressee: &'marker BearerTokenAddressee,
        container_bindings: &'marker ContainerBindings,
    },
    Unavailable,
}
