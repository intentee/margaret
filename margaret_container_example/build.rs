fn main() {
    margaret_container::build(env!("CARGO_MANIFEST_DIR"))
        .expect("the example container is generated");
}
