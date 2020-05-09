use bytes::Bytes;
use nebula_status::{Status, StatusCode, StatusData};
use serde::Serialize;
use warp::Rejection;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_server_error_is_500_and_has_message() {
        let msg = "an error message";
        let rej = server_error_into_rejection(msg.to_string());

        let status: Status<Error> = Status::recover(rej)
            .expect("rejection should contain a status");

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

/// The application error type. This exists primarily to enable serialization
/// into the appropriate JSON format.
#[derive(Serialize, Clone, Debug)]
pub(crate) struct Error {
    /// The associated message for this error. May be displayed to the client
    /// and/or logged somewhere.
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

/// The application success type. Contains something that can be serialized
/// into JSON. The wrapper exists to enable serialization into the proper
/// JSON format.
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
