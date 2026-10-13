use margaret_attributes::canonical_path::CanonicalPath;
use margaret_model::on_delete::OnDelete;

use crate::collected_column::CollectedColumn;

pub(crate) enum CollectedFieldShape {
    Column(CollectedColumn),
    Key {
        column_base: String,
        on_delete: OnDelete,
        target: CanonicalPath,
    },
}
