use uuid::Uuid;

use margaret::framework::active_record::key::Key;
use margaret::framework::macros::model;
use margaret::framework::model::column_default::ColumnDefault;
use margaret::framework::model::on_delete::OnDelete;

#[model(table = "nodes")]
#[index(name = "nodes_by_parent", fields = [parent, id])]
#[has_many(name = "children", model = Node, key = parent)]
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    #[column(primary_key, default = ColumnDefault::UuidV7)]
    pub id: Uuid,
    #[column]
    pub label: String,
    #[column]
    #[foreign_key(on_delete = OnDelete::SetNull)]
    pub parent: Option<Key<Node>>,
}
