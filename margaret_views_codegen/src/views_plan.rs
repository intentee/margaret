use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container::container_bindings::ContainerBindings;

use crate::view::View;
use crate::views::views;
use crate::views_codegen_error::ViewsCodegenError;

pub struct ViewsPlan {
    pub(crate) console_arguments: Vec<ConsoleArgument>,
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
        let console_arguments = bindings.serve_arguments(&retained_roots, &[])?;

        Ok(Self {
            console_arguments,
            retained_roots,
            views,
        })
    }
}
