use serde::Serialize;
use serde::Serializer;
use serde::ser::Error;

pub struct FailingChunk;

impl Serialize for FailingChunk {
    fn serialize<Target: Serializer>(
        &self,
        _serializer: Target,
    ) -> Result<Target::Ok, Target::Error> {
        Err(Target::Error::custom(
            "this response payload cannot be serialized",
        ))
    }
}
