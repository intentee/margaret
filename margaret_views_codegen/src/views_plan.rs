use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;

use crate::view::View;
use crate::views::views;
use crate::views_codegen_error::ViewsCodegenError;

pub struct ViewsPlan {
    pub(crate) retained_roots: Vec<CanonicalPath>,
    pub(crate) views: Vec<View>,
}

impl ViewsPlan {
    /// # Errors
    ///
    /// Returns `ViewsCodegenError` propagated from the work it performs.
    pub fn build(index: &AttributeIndex) -> Result<Self, ViewsCodegenError> {
        let views = views(index)?;
        let retained_roots = views
            .iter()
            .map(|view| view.concrete_path.clone())
            .collect::<Vec<_>>();
        Ok(Self {
            retained_roots,
            views,
        })
    }
}
