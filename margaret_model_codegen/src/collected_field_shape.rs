use margaret_attributes::canonical_path::CanonicalPath;
use margaret_model::on_delete::OnDelete;

use crate::collected_column::CollectedColumn;

pub(crate) enum CollectedFieldShape {
    Column(CollectedColumn),
    Key {
        on_delete: OnDelete,
        target: CanonicalPath,
    },
}
