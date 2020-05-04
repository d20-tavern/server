use bytes::Bytes;
use nebula_status::{Status, StatusCode, StatusData};
use serde::Serialize;
use warp::Rejection;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

pub(crate) fn serialize_to_bytes(data: impl Serialize) -> Bytes {
    serde_json::to_string(&data).unwrap().into()
}

pub(crate) fn server_error_into_rejection(msg: String) -> Rejection {
    Status::with_data(&StatusCode::INTERNAL_SERVER_ERROR, Error::new(msg)).into()
}

#[derive(Serialize, Clone, Debug)]
pub(crate) struct Error {
    pub(crate) message: String,
}

impl Error {
    pub(crate) fn new(msg: String) -> Self {
        Self { message: msg }
    }
}

// into() calls unwrap(). This will only happen if Serialize for String
// fails for some reason. Judging by skimming the source for serde_json,
// this will only occur if escaping and formatting the string fails.
// For the time being, and for simplicity, this seems like a rare enough
// occasion to warrant an unwrap().
impl From<Error> for Bytes {
    fn from(err: Error) -> Self {
        serialize_to_bytes(&err)
    }
}

#[derive(Serialize, Clone, Debug)]
pub(crate) struct Success<T: Serialize + StatusData> {
    pub(crate) data: T,
}

impl<T: Serialize + StatusData> Success<T> {
    pub(crate) fn new(data: T) -> Self {
        Self { data }
    }
}

// See note about unsafe() on Error. That should also apply here.
impl<T: Serialize + StatusData> From<Success<T>> for Bytes {
    fn from(suc: Success<T>) -> Self {
        serialize_to_bytes(&suc)
    }
}
