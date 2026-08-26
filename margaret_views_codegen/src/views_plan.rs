use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::container_bindings::ContainerBindings;
use margaret_serve_input_codegen::serve_input::ServeInput;

use crate::view::View;
use crate::views::views;
use crate::views_codegen_error::ViewsCodegenError;

pub struct ViewsPlan {
    pub(crate) serve_inputs: Vec<ServeInput>,
    pub(crate) retained_roots: Vec<CanonicalPath>,
    pub(crate) views: Vec<View>,
}

impl ViewsPlan {
    /// # Errors
    ///
    /// Returns `ViewsCodegenError` propagated from the work it performs.
    pub fn build(
        index: &AttributeIndex,
        bindings: &ContainerBindings,
    ) -> Result<Self, ViewsCodegenError> {
        let views = views(index)?;
        let retained_roots = views
            .iter()
            .map(|view| view.concrete_path.clone())
            .collect::<Vec<_>>();
        let serve_inputs = bindings.serve_inputs(&retained_roots, &[])?;

        Ok(Self {
            serve_inputs,
            retained_roots,
            views,
        })
    }
}
