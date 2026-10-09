use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::node::Node;
use margaret_active_record_tests::models::node_with_parent::NodeWithParent;

use crate::postgres::created_node::created_node;
use crate::postgres::started_with_models::started_with_models;

#[tokio::test]
async fn loads_an_absent_optional_parent() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let root = created_node(database, "root", None).await;

    assert_eq!(
        Node::query()
            .id
            .eq(root.id)
            .load::<NodeWithParent, _>(database)
            .await
            .expect("the node is loaded"),
        Lookup::Found(NodeWithParent {
            node: root,
            parent: None,
        })
    );
}
