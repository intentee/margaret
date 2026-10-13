use margaret_model_codegen::model_field::ModelField;

use crate::trie_builder::TrieBuilder;

pub(crate) struct BuilderEdge<'model> {
    pub(crate) field: &'model ModelField,
    pub(crate) node: TrieBuilder<'model>,
}
