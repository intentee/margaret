use margaret_container::container_bindings::ContainerBindings;
use margaret_tag_codegen::session_source::SessionSource;

#[derive(Clone, Copy)]
pub(crate) enum SessionMarker<'marker> {
    Scanned {
        container_bindings: &'marker ContainerBindings,
        source: SessionSource,
    },
    Unavailable,
}
