use margaret::framework::codegen::codegen_error::CodegenError;
use margaret::framework::codegen::generate;

fn main() -> Result<(), CodegenError> {
    generate::generate()
}
