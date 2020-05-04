use crate::status::Error;
use nebula_form::Form;
use nebula_status::{Status, StatusCode};
use warp::Rejection;

pub(crate) fn missing_field_error(field_name: &str) -> Rejection {
    let err = Status::with_data(
        &StatusCode::BAD_REQUEST,
        Error::new(format!("field {} is missing", field_name)),
    );
    warp::reject::custom(err)
}

pub(crate) fn field_is_file_error(field_name: &str) -> Rejection {
    let err = Status::with_data(
        &StatusCode::BAD_REQUEST,
        Error::new(format!("field {} was a file, expected text", field_name)),
    );
    warp::reject::custom(err)
}

pub(crate) fn get_form_text_field(form: &Form, field_name: &str) -> Result<String, Rejection> {
    form.get(field_name)
        .ok_or(missing_field_error(field_name))?
        .as_text()
        .map(|txt| txt.to_string())
        .ok_or_else(|| field_is_file_error(field_name))
}
