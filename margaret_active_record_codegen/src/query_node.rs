use crate::node_ending::NodeEnding;
use crate::node_ordering::NodeOrdering;
use crate::query_edges::QueryEdges;

pub(crate) enum QueryNode<'model> {
    Branch(QueryEdges<'model>),
    Leaf(NodeEnding),
}

impl<'model> QueryNode<'model> {
    pub(crate) fn ordering(&self) -> NodeOrdering<'model> {
        match self {
            Self::Leaf(NodeEnding::UniqueEnd) => NodeOrdering::Total(Vec::new()),
            Self::Leaf(NodeEnding::PlainEnd) => NodeOrdering::Partial,
            Self::Branch(edges) => match edges.rest.as_slice() {
                [] if !edges.first.field.nullable => match edges.first.node.ordering() {
                    NodeOrdering::Total(rest) => {
                        NodeOrdering::Total([edges.first.field].into_iter().chain(rest).collect())
                    }
                    NodeOrdering::Partial => NodeOrdering::Partial,
                },
                _ => NodeOrdering::Partial,
            },
        }
    }
}
