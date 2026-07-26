use margaret_container_error::construction_error::ConstructionError;

fn main() {
    let outcome: Result<u8, std::io::Error> = Ok(0);

    let _ = ConstructionError::wrap("crate::keys::KeyLoader", outcome);
}
