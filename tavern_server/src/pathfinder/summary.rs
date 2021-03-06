use super::Links;
use serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::marker::PhantomData;
pub use tavern_derive::Summarize;
use uuid::Uuid;

use crate::db::{self, Connection, Delete, DeleteById, TryFromDb, Error, GetById, GetAll};

pub trait Summarize<T> {
    fn id(&self) -> &Uuid;
    fn links(&self) -> Option<&Links>;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}

impl<T, U: Summarize<T> + ?Sized> From<&U> for Summary<T> {
    fn from(other: &U) -> Self {
        Self {
            id: other.id().clone(),
            links: other.links().cloned(),
            name: other.name().to_string(),
            description: other.description().to_string(),
            phantom: Default::default(),
        }
    }
}

impl<T> Summarize<T> for Summary<T> {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn links(&self) -> Option<&Links> {
        self.links.as_ref()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }
}

impl<T> Ord for dyn Summarize<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name()
            .cmp(other.name())
            .then_with(|| self.id().cmp(other.id()))
    }
}

impl<T> PartialOrd for dyn Summarize<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialEq for dyn Summarize<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl<T> Eq for dyn Summarize<T> {}

#[derive(Serialize, Deserialize, Clone)]
pub struct Summary<T> {
    id: Uuid,
    links: Option<Links>,
    name: String,
    description: String,
    #[serde(skip)]
    phantom: PhantomData<T>,
}

impl<T> Ord for Summary<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name
            .cmp(&other.name)
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl<T> PartialOrd for Summary<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialEq for Summary<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for Summary<T> {}

impl<T> Delete for Summary<T> where T: Summarize<T> + DeleteById + Sized {
    fn db_delete(&self, conn: &Connection) -> Result<(), db::Error> {
        T::db_delete_by_id(self.id(), conn)
    }
}

impl<T> TryFromDb for Summary<T> where T: Summarize<T> + TryFromDb + Sized {
    type DBType = T::DBType;

    fn try_from_db(other: Self::DBType, conn: &Connection) -> Result<Self, Error> where Self: Sized {
        Ok(Self::from(&T::try_from_db(other, conn)?))
    }
}
