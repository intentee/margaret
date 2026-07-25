use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::tag::Tag;

pub enum FrameworkConstruction {
    Accessor {
        accessor: String,
        source: CanonicalPath,
    },
    Constructor {
        endpoints: Vec<Tag>,
        is_async: bool,
        method: String,
    },
    Unit,
}
