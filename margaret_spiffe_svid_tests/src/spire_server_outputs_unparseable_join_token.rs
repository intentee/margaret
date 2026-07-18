use std::path::PathBuf;

use crate::fixtures_dir::fixtures_dir;

#[must_use]
pub fn spire_server_outputs_unparseable_join_token() -> PathBuf {
    fixtures_dir().join("spire_server_outputs_unparseable_join_token.sh")
}
