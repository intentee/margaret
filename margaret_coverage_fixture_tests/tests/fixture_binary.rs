use std::path::Path;

pub fn fixture_binary() -> &'static Path {
    Path::new(env!("CARGO_BIN_EXE_margaret_coverage_fixture_cli"))
}
