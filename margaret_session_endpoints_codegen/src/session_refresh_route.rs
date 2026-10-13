use margaret_attributes::canonical_path::CanonicalPath;

pub enum SessionRefreshRoute {
    Served { route: CanonicalPath },
    Unserved,
}
