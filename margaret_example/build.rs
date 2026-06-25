fn main() {
    margaret_codegen::build::build(env!("CARGO_MANIFEST_DIR")).expect("the example is generated");
}
