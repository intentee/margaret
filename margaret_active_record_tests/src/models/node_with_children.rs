use margaret::framework::active_record::children::Children;
use margaret::framework::macros::eager_load;

use crate::models::node::Node;

#[eager_load(model = Node)]
#[derive(Debug, PartialEq)]
pub struct NodeWithChildren {
    #[base]
    pub node: Node,
    #[relation(children, limit = 5)]
    pub children: Children<Node>,
}
