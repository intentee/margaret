fn main() {
    margaret_http_codegen::build(env!("CARGO_MANIFEST_DIR"))
        .expect("the example server is generated");
}
