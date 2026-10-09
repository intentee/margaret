use margaret_model_codegen::index_kind::IndexKind;
use margaret_model_codegen::model::Model;
use margaret_model_codegen::model_field::ModelField;
use margaret_model_codegen::model_index::ModelIndex;

use crate::builder_edge::BuilderEdge;
use crate::node_ending::NodeEnding;
use crate::query_edge::QueryEdge;
use crate::query_edges::QueryEdges;
use crate::query_node::QueryNode;

fn identifies_a_row(model: &Model, index: &ModelIndex) -> bool {
    index.kind != IndexKind::Plain
        || model
            .indexes
            .all()
            .filter(|key| key.kind != IndexKind::Plain)
            .any(|key| key.fields.iter().all(|field| index.fields.contains(field)))
}

pub(crate) struct TrieBuilder<'model> {
    children: Vec<BuilderEdge<'model>>,
    ends_on_a_row: bool,
}

impl<'model> TrieBuilder<'model> {
    pub(crate) fn of(model: &'model Model) -> Self {
        let mut root = Self::empty();

        for index in model.indexes.all() {
            root.insert(&index.fields, identifies_a_row(model, index));
        }

        root
    }

    pub(crate) fn root_edges(self) -> Vec<QueryEdge<'model>> {
        self.sorted_edges().collect()
    }

    fn built(self) -> QueryNode<'model> {
        let ending = if self.ends_on_a_row {
            NodeEnding::UniqueEnd
        } else {
            NodeEnding::PlainEnd
        };
        let mut edges = self.sorted_edges();

        match edges.next() {
            Some(first) => QueryNode::Branch(QueryEdges {
                first: Box::new(first),
                rest: edges.collect(),
            }),
            None => QueryNode::Leaf(ending),
        }
    }

    fn sorted_edges(mut self) -> impl Iterator<Item = QueryEdge<'model>> {
        self.children.sort_by_key(|edge| edge.field.start);
        self.children
            .into_iter()
            .map(|BuilderEdge { field, node }| QueryEdge {
                field,
                node: node.built(),
            })
    }

    fn empty() -> Self {
        Self {
            children: Vec::new(),
            ends_on_a_row: false,
        }
    }

    fn insert(&mut self, path: &'model [ModelField], identifies_a_row: bool) {
        let Some((first, rest)) = path.split_first() else {
            self.ends_on_a_row |= identifies_a_row;

            return;
        };

        if let Some(edge) = self.children.iter_mut().find(|edge| edge.field == first) {
            edge.node.insert(rest, identifies_a_row);
        } else {
            let mut node = Self::empty();

            node.insert(rest, identifies_a_row);
            self.children.push(BuilderEdge { field: first, node });
        }
    }
}
