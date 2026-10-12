use margaret::framework::macros::service;

#[service]
struct Worker {
    value: String,
}
