use bytes::Bytes;

pub(crate) enum ResponseBody {
    Failed(reqwest::Error),
    Oversized { max_bytes: usize },
    Read(Bytes),
}
