use margaret::framework::macros::singleton;

#[singleton]
struct Lonely {
    value: String,
}
