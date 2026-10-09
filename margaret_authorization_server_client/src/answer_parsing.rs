use serde::de::DeserializeOwned;

pub(crate) enum AnswerParsing<TAnswer> {
    Malformed(serde_path_to_error::Error<serde_json::Error>),
    Parsed(TAnswer),
}

impl<TAnswer: DeserializeOwned> AnswerParsing<TAnswer> {
    pub(crate) fn of(body: &[u8]) -> Self {
        match serde_path_to_error::deserialize(&mut serde_json::Deserializer::from_slice(body)) {
            Ok(answer) => Self::Parsed(answer),
            Err(source) => Self::Malformed(source),
        }
    }
}
