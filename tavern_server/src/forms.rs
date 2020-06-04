use crate::status::{self, Error};
use nebula_form::Form;
use nebula_status::{Status, StatusCode};
use warp::Rejection;
use uuid::Uuid;
use std::str::FromStr;
use crate::db::{Connection, GetById, Error as DBError};
use diesel::result::Error as DieselError;

#[cfg(test)]
mod tests {
    use nebula_form::{Field, Form, FormFile};

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
        form.insert(
            FIELD_FILE_BAZ,
            Field::File(FormFile {
                filename: VALUE_FILE_BAZ_NAME.to_string(),
                content_type: VALUE_FILE_BAZ_TYPE.to_string(),
                bytes: VALUE_FILE_BAZ_CONTENT.into(),
            }),
        );
        form
    }

    #[test]
    fn get_form_text_field_works() {
        let form = generate_form();
        let value: String =
            super::get_required_form_text_field(&form, FIELD_TEXT_FOO).expect("this should not fail");
        assert_eq!(&value, VALUE_TEXT_FOO);
    }

    #[test]
    fn get_file_field_as_text_fails() {
        let form = generate_form();
        match super::get_required_form_text_field::<String>(&form, FIELD_FILE_BAZ) {
            Ok(_) => assert!(false, "file as text should not have returned successfully"),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn get_missing_field_as_text_fails() {
        let form = generate_form();
        match super::get_required_form_text_field::<String>(&form, FIELD_MISSING_OOPS) {
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
pub(crate) fn get_optional_form_text_field<T: FromStr>(form: &Form, field_name: &str) -> Result<Option<T>, Rejection> {
    form.get(field_name)
        .map(|val| {
            val.as_text()
                .map(|txt| {
                    txt.parse()
                })
                .transpose()
                .map_err(|_| field_is_file_error(field_name))
        }).transpose()
        .map(|opt| opt.flatten())
}

/// Retrieve the specified *text* field from the form or return a relevant
/// error.
pub(crate) fn get_required_form_text_field<T: FromStr>(form: &Form, field_name: &str) -> Result<T, Rejection> {
    get_optional_form_text_field(form, field_name)?
        .ok_or_else(|| missing_field_error(field_name))
}

pub(crate) fn value_by_id<T: GetById>(id: Uuid, conn: &Connection) -> Result<T, Rejection> {
    T::db_get_by_id(&id, conn)
        .map_err(|err| {
            match err {
                DBError::RunQuery(err) => {
                    match err {
                        DieselError::NotFound => {
                            let error = Error::new(format!{"invalid id: {}", id});
                            Status::with_data(&StatusCode::BAD_REQUEST, error).into()
                        },
                        err => status::server_error_into_rejection(err.to_string())
                    }
                },
                err => status::server_error_into_rejection(err.to_string()),
            }
        })
}

pub(crate) fn valid_id<T: GetById>(id: Uuid, conn: &Connection) -> Result<Uuid, Rejection> {
    value_by_id::<T>(id, conn)
        .map(|_| id)
}

pub(crate) fn valid_id_or_new<T: GetById>(id: Option<Uuid>, conn: &Connection) -> Result<Uuid, Rejection> {
    match id {
        None => Ok(Uuid::new_v4()),
        Some(id) => valid_id::<T>(id, conn)
    }
}

pub(crate) fn db_error_to_rejection(err: DBError, field: &str) -> Rejection {
    match err {
        DBError::RunQuery(err) => {
            match err {
                DieselError::NotFound => field_is_invalid_error(field),
                err => Rejection::from(DBError::RunQuery(err)),
            }
        },
        err => Rejection::from(err),
    }
}

pub trait TryFromForm {
    fn try_from_form(conn: &Connection, form: Form, this_id: Option<Uuid>, parent_id: Option<Uuid>) -> Result<Self, Rejection> where Self: Sized;
}