use std::path::Path;

use margaret_attributes::crate_root::CrateRoot;
use margaret_authorization_grants_schema::authorization_grants_schema_directory::AUTHORIZATION_GRANTS_SCHEMA_DIRECTORY;
use margaret_client_assertions_schema::client_assertions_schema_directory::CLIENT_ASSERTIONS_SCHEMA_DIRECTORY;
use margaret_signing_keys_schema::signing_keys_schema_directory::SIGNING_KEYS_SCHEMA_DIRECTORY;

use crate::authorization_grants_store_canonical_path::authorization_grants_store_canonical_path;
use crate::client_assertions_store_canonical_path::client_assertions_store_canonical_path;
use crate::framework_state_store::FrameworkStateStore;
use crate::signing_keys_store_canonical_path::signing_keys_store_canonical_path;

pub(crate) fn framework_state_stores() -> [FrameworkStateStore; 3] {
    [
        FrameworkStateStore {
            provided: authorization_grants_store_canonical_path(),
            schema: CrateRoot::new(
                "margaret_authorization_grants_schema",
                Path::new(AUTHORIZATION_GRANTS_SCHEMA_DIRECTORY),
            ),
        },
        FrameworkStateStore {
            provided: client_assertions_store_canonical_path(),
            schema: CrateRoot::new(
                "margaret_client_assertions_schema",
                Path::new(CLIENT_ASSERTIONS_SCHEMA_DIRECTORY),
            ),
        },
        FrameworkStateStore {
            provided: signing_keys_store_canonical_path(),
            schema: CrateRoot::new(
                "margaret_signing_keys_schema",
                Path::new(SIGNING_KEYS_SCHEMA_DIRECTORY),
            ),
        },
    ]
}
