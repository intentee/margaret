use margaret::framework::macros::eager_load;

use crate::models::node::Node;

#[eager_load(model = Node)]
#[derive(Debug, PartialEq)]
pub struct NodeWithParent {
    #[base]
    pub node: Node,
    #[relation(parent)]
    pub parent: Option<Node>,
}
