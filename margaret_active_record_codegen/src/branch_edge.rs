use margaret_model_codegen::model_field::ModelField;

use crate::query_edges::QueryEdges;

pub(crate) struct BranchEdge<'tree, 'model> {
    pub(crate) edges: &'tree QueryEdges<'model>,
    pub(crate) field: &'model ModelField,
}
