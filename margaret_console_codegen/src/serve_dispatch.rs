use margaret_service_codegen::serve_invocation_read::ServeInvocationRead;

pub enum ServeDispatch {
    Served(ServeInvocationRead),
    Unserved,
}
