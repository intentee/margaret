use margaret_codegen::codegen_error::CodegenError;
use margaret_codegen::generate_framework_models::generate_framework_models;

fn main() -> Result<(), CodegenError> {
    generate_framework_models()
}
