use uuid::Uuid;
use crate::db::Connection;
use warp::{Filter, Rejection, Reply};
use warp::filters::BoxedFilter;
use crate::{db, forms};
use nebula_status::{Status, StatusCode, StatusData, StatusInnerData};
use serde::{Serialize, Deserialize};
use crate::status::Success;
use http::Response;
use nebula_form::Form;
pub use tavern_derive::Filters;
use serde::de::DeserializeOwned;

pub trait APIPath {
    fn api_path() -> BoxedFilter<(Uuid,)>;
}

pub trait GetById {
    fn get_by_id(parent_id: Option<Uuid>) -> BoxedFilter<(Box<dyn Reply>,)>;
}

impl<T> GetById for T where T: db::GetById + Serialize + DeserializeOwned + StatusInnerData {
    fn get_by_id(_parent_id: Option<Uuid>) -> BoxedFilter<(Box<dyn Reply>,)> where Self: Sized + Send {
        async fn get_item_by_id<U>((conn, this_id): (Connection, Uuid)) -> Result<Box<dyn Reply>, Rejection> where U: db::GetById + Serialize + DeserializeOwned + StatusInnerData {
            let result =  U::db_get_by_id(&this_id, &conn)
                .map_err(Rejection::from)?;
            Ok(Box::new(Response::from(Status::with_data(&StatusCode::OK, Success::<U>::new(result)))))
        }

        warp::path::param()
            .and(warp::filters::method::get())
            .and(db::conn_filter())
            .map(move |this_id, conn| { (conn, this_id) })
            .and_then(get_item_by_id::<Self>)
            .boxed()
    }
}

pub trait GetAll {
    fn get_all(parent_id: Option<Uuid>) -> BoxedFilter<(Box<dyn Reply>,)> where Self: Sized + Send;
}

impl<T> GetAll for T where T: db::GetAll + Serialize + DeserializeOwned + StatusInnerData {
    fn get_all(parent_id: Option<Uuid>) -> BoxedFilter<(Box<dyn Reply>,)> where Self: Sized + Send {
        async fn get_all_items<U>((conn, parent_id): (Connection, Option<Uuid>)) -> Result<Box<dyn Reply>, Rejection> where U: db::GetAll + Serialize + DeserializeOwned + StatusInnerData {
            let result = U::db_get_all(&conn)
                .map_err(Rejection::from)?;
            Ok(Box::new(Response::from(Status::with_data(&StatusCode::OK, Success::<Vec<U>>::new(result)))))
        }

        warp::filters::method::get()
            .and(db::conn_filter())
            .map(move |conn| (conn, parent_id))
            .and_then(get_all_items::<Self>)
            .boxed()
    }
}

pub trait Insert {
    fn insert(parent_id: Option<Uuid>) -> BoxedFilter<(Box<dyn Reply>,)>;
}


impl<T> Insert for T where T: db::Insert + Serialize + DeserializeOwned + StatusInnerData + forms::TryFromForm + 'static {
    fn insert(parent_id: Option<Uuid>) -> BoxedFilter<(Box<dyn Reply>,)> {
        async fn insert_item<U>((conn, form, parent_id): (Connection, Form, Option<Uuid>)) -> Result<Box<dyn Reply>, Rejection> where U: db::Insert + Serialize + DeserializeOwned + StatusInnerData + forms::TryFromForm + 'static {
            let item = U::try_from_form(&conn, form, None, parent_id)?;
            item.db_insert(&conn)?;
            Ok(Box::new(Status::with_data(&StatusCode::OK, Success::new(item))))
        }

        warp::filters::method::post()
            .and(nebula_form::form_filter())
            .and(db::conn_filter())
            .map(move |form, conn| (conn, form, parent_id))
            .and_then(insert_item::<Self>)
            .boxed()
    }
}

pub trait Update {
    fn update(parent_id: Option<Uuid>) -> BoxedFilter<(Box<dyn Reply>,)>;
}

impl<T> Update for T where T: db::Update + forms::TryFromForm + 'static {
    fn update(parent_id: Option<Uuid>) -> BoxedFilter<(Box<dyn Reply>,)> {
        async fn update_item<U>((conn, form, this_id, parent_id): (Connection, Form, Uuid, Option<Uuid>)) -> Result<Box<dyn Reply>, Rejection> where U: db::Update + forms::TryFromForm + 'static {
            let item = U::try_from_form(&conn, form, Some(this_id), parent_id)?;
            item.db_update(&conn)?;
            Ok(Box::new(Status::new(&StatusCode::OK)))
        }

        warp::path::param()
            .and(warp::filters::method::put())
            .and(nebula_form::form_filter())
            .and(db::conn_filter())
            .map(move |this_id, form, conn| (conn, form, this_id, parent_id))
            .and_then(update_item::<Self>)
            .boxed()
    }
}

pub trait DeleteById {
    fn delete_by_id(parent_id: Option<Uuid>) -> BoxedFilter<(Box<dyn Reply>,)>;
}

impl<T> DeleteById for T where T: db::DeleteById + 'static {
    fn delete_by_id(parent_id: Option<Uuid>) -> BoxedFilter<(Box<dyn Reply>,)> {
        async fn delete_item<U>((conn, this_id, _parent_id): (Connection, Uuid, Option<Uuid>)) -> Result<Box<dyn Reply>, Rejection> where U: db::DeleteById + 'static {
            U::db_delete_by_id(&this_id, &conn)?;
            Ok(Box::new(Status::new(&StatusCode::OK)))
        }

        warp::path::param()
            .and(warp::filters::method::delete())
            .and(db::conn_filter())
            .map(move |this_id, conn| (conn, this_id, parent_id))
            .and_then(delete_item::<Self>)
            .boxed()
    }
}

pub trait Filters {
    fn filters(parent_id: Option<Uuid>) -> BoxedFilter<(Box<dyn Reply>,)>;
}
