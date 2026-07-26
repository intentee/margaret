use margaret_container_error::construction_error::ConstructionError;

fn main() {
    let _ = ConstructionError::wrap("crate::keys::KeyLoader", 5u8);
}
