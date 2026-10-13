use margaret_model_codegen::model_field::ModelField;

use crate::query_node::QueryNode;

pub(crate) struct QueryEdge<'model> {
    pub(crate) field: &'model ModelField,
    pub(crate) node: QueryNode<'model>,
}
