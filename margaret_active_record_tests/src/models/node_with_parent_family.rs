use margaret::framework::macros::eager_load;

use crate::models::node::Node;
use crate::models::node_with_children::NodeWithChildren;

#[eager_load(model = Node)]
#[derive(Debug, PartialEq)]
pub struct NodeWithParentFamily {
    #[base]
    pub node: Node,
    #[relation(parent)]
    pub parent: Option<NodeWithChildren>,
}
