use bytes::Bytes;
use nebula_status::{Status, StatusCode, StatusData, StatusInnerData};
use serde::Serialize;
use warp::Rejection;
use std::fmt::Debug;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_server_error_is_500_and_has_message() {
        let msg = "an error message";
        let rej = server_error_into_rejection(msg.to_string());

        let status: Status<Error> =
            Status::recover(rej).expect("rejection should contain a status");

        assert_eq!(status.code(), &StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status.data().unwrap().message, msg);
    }
}

/// Transform a serializable type into bytes.
pub(crate) fn serialize_to_bytes(data: impl Serialize) -> Bytes {
    serde_json::to_string(&data).unwrap().into()
}

/// Generates a Rejection containing a Status<Error> created using the given
/// error message.
pub(crate) fn server_error_into_rejection(msg: String) -> Rejection {
    Status::with_data(&StatusCode::INTERNAL_SERVER_ERROR, Error::new(msg)).into()
}

pub(crate) fn invalid_header_error(header: &str) -> Rejection {
    Status::with_data(
        &StatusCode::BAD_REQUEST,
        Error::new(format!("invalid header: {}", header)),
    )
    .into()
}

pub(crate) fn not_found() -> Rejection {
    Status::new(&StatusCode::NOT_FOUND).into()
}

pub(crate) fn not_authorized() -> Rejection {
    Status::new(&StatusCode::UNAUTHORIZED).into()
}

/// The application error type. This exists primarily to enable serialization
/// into the appropriate JSON format.
#[derive(Serialize, Clone, Debug)]
pub struct Error {
    /// The associated message for this error. May be displayed to the client
    /// and/or logged somewhere.
    pub message: String,
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

/// The application success type. Contains something that can be serialized
/// into JSON. The wrapper exists to enable serialization into the proper
/// JSON format.
#[derive(Serialize, Clone, Debug)]
pub struct Success<T: Serialize + StatusInnerData> {
    pub data: T,
}

impl<T: Serialize + StatusInnerData> Success<T> {
    pub(crate) fn new(data: T) -> Self {
        Self { data }
    }
}

// See note about unsafe() on Error. That should also apply here.
impl<T: Serialize + StatusInnerData> From<Success<T>> for Bytes {
    fn from(suc: Success<T>) -> Self {
        serialize_to_bytes(&suc)
    }
}
