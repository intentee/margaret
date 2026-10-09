use margaret::framework::macros::eager_load;

use crate::models::node::Node;
use crate::models::node_with_parent::NodeWithParent;

#[eager_load(model = Node)]
#[derive(Debug, PartialEq)]
pub struct NodeWithGrandparent {
    #[base]
    pub node: Node,
    #[relation(parent)]
    pub parent: Option<NodeWithParent>,
}
