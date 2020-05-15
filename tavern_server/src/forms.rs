use crate::status::Error;
use nebula_form::Form;
use nebula_status::{Status, StatusCode};
use warp::Rejection;

mod tests {
    use super::*;
    use nebula_form::{Form, FormFile, Field};

    const FIELD_MISSING_OOPS: &str = "oops";
    const FIELD_TEXT_FOO: &str = "foo";
    const VALUE_TEXT_FOO: &str = "bar value";
    const FIELD_FILE_BAZ: &str = "baz";
    const VALUE_FILE_BAZ_NAME: &str = "baz.txt";
    const VALUE_FILE_BAZ_TYPE: &str = "text/plain";
    const VALUE_FILE_BAZ_CONTENT: &str = "file contents";

    fn generate_form() -> Form {
        let mut form = Form::new();
        form.insert(FIELD_TEXT_FOO, Field::Text(VALUE_TEXT_FOO.to_string()));
        form.insert(FIELD_FILE_BAZ, Field::File(FormFile {
            filename: VALUE_FILE_BAZ_NAME.to_string(),
            content_type: VALUE_FILE_BAZ_TYPE.to_string(),
            bytes: VALUE_FILE_BAZ_CONTENT.into(),
        }));
        form
    }

    #[test]
    fn get_form_text_field_works() {
        let form = generate_form();
        let value = get_form_text_field(&form, FIELD_TEXT_FOO).expect("this should not fail");
        assert_eq!(&value, VALUE_TEXT_FOO);
    }

    #[test]
    fn get_file_field_as_text_fails() {
        let form = generate_form();
        match get_form_text_field(&form, FIELD_FILE_BAZ) {
            Ok(_) => assert!(false, "file as text should not have returned successfully"),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn get_missing_field_as_text_fails() {
        let form = generate_form();
        match get_form_text_field(&form, FIELD_MISSING_OOPS) {
            Ok(_) => assert!(false, "getting a missing field should not succeed"),
            Err(_) => assert!(true),
        }
    }
}

/// Generate an HTTP 400 Rejection indicating that the required field is
/// missing.
pub(crate) fn missing_field_error(field_name: &str) -> Rejection {
    let err = Status::with_data(
        &StatusCode::BAD_REQUEST,
        Error::new(format!("field {} is missing", field_name)),
    );
    warp::reject::custom(err)
}

/// Generate an HTTP 400 Rejection indicating that the required field was
/// expected to be text but instead was a file.
pub(crate) fn field_is_file_error(field_name: &str) -> Rejection {
    let err = Status::with_data(
        &StatusCode::BAD_REQUEST,
        Error::new(format!("field {} was a file, expected text", field_name)),
    );
    warp::reject::custom(err)
}

/// Generate an HTTP 400 Rejection indicating that the required field contains
/// an invalid value.
pub(crate) fn field_is_invalid_error(field_name: &str) -> Rejection {
    let err = Status::with_data(
        &StatusCode::BAD_REQUEST,
        Error::new(format!("field {} is invalid", field_name)),
    );
    warp::reject::custom(err)
}

/// Retrieve the specified *text* field from the form or return a relevant
/// error.
pub(crate) fn get_form_text_field(form: &Form, field_name: &str) -> Result<String, Rejection> {
    form.get(field_name)
        .ok_or(missing_field_error(field_name))?
        .as_text()
        .map(|txt| txt.to_string())
        .ok_or_else(|| field_is_file_error(field_name))
}
