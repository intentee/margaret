use margaret_model_codegen::model_field::ModelField;

use crate::query_edges::QueryEdges;
use crate::query_node::QueryNode;

pub(crate) struct BranchEdge<'tree, 'model> {
    pub(crate) edges: &'tree QueryEdges<'model>,
    pub(crate) field: &'model ModelField,
    pub(crate) node: &'tree QueryNode<'model>,
}
