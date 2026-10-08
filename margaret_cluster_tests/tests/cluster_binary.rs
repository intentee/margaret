use std::path::Path;

pub fn cluster_binary() -> &'static Path {
    Path::new(env!("CARGO_BIN_EXE_margaret_cluster_instance"))
}
