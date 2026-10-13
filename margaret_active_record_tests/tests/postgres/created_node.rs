use margaret::framework::active_record::creatable::Creatable;
use margaret::framework::active_record::key::Key;
use margaret::framework::database::database::Database;
use margaret_active_record_tests::margaret::models::models_node_node::draft::Draft;
use margaret_active_record_tests::models::node::Node;

pub async fn created_node(database: &Database, label: &str, parent: Option<&Node>) -> Node {
    Node::create(Draft {
        label: label.to_string(),
        parent: parent.map(Key::of),
    })
    .run(database)
    .await
    .expect("the node is created")
}
