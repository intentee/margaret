use margaret::framework::active_record::children::Children;
use margaret::framework::active_record::completeness::Completeness;
use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::node::Node;
use margaret_active_record_tests::models::node_with_children::NodeWithChildren;
use margaret_active_record_tests::models::node_with_parent_family::NodeWithParentFamily;

use crate::postgres::created_node::created_node;
use crate::postgres::started_with_models::started_with_models;

#[tokio::test]
async fn loads_the_family_of_a_present_parent() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let root = created_node(database, "root", None).await;
    let leaf = created_node(database, "leaf", Some(&root)).await;

    assert_eq!(
        Node::query()
            .id
            .eq(leaf.id)
            .load::<NodeWithParentFamily, _>(database)
            .await
            .expect("the node is loaded"),
        Lookup::Found(NodeWithParentFamily {
            node: leaf.clone(),
            parent: Some(NodeWithChildren {
                node: root,
                children: Children {
                    completeness: Completeness::Complete,
                    records: vec![leaf],
                },
            }),
        })
    );
}
