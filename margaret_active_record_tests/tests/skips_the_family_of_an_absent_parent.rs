use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::node::Node;
use margaret_active_record_tests::models::node_with_parent_family::NodeWithParentFamily;

use crate::created_node::created_node;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn skips_the_family_of_an_absent_parent() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let root = created_node(database, "root", None).await;

    assert_eq!(
        Node::query()
            .id
            .eq(root.id)
            .load::<NodeWithParentFamily, _>(database)
            .await
            .expect("the node is loaded"),
        Lookup::Found(NodeWithParentFamily {
            node: root,
            parent: None,
        })
    );
}
