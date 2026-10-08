use margaret_attributes::canonical_path::CanonicalPath;

use crate::key_column::KeyColumn;

pub(crate) enum PrimaryKeySource {
    Column(KeyColumn),
    Key { target: CanonicalPath },
}
