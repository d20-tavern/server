use async_trait::async_trait;
use crate::Links;
use serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::marker::PhantomData;
#[cfg(feature = "tavern")]
use sqlx::postgres::PgRow;
#[cfg(feature = "tavern")]
use tavern_db::{self, TryFromRow, TryFromUuid};
pub use tavern_derive::Summarize;
use uuid::Uuid;

pub trait Summarize<T> {
    fn id(&self) -> &Uuid;
    fn links(&self) -> Option<&Links>;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}

#[derive(Serialize, Deserialize)]
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

#[cfg(feature = "tavern")]
#[async_trait]
impl<T: TryFromRow + Summarize<T> + 'static> TryFromRow for Summary<T> {
    async fn try_from_row(row: &PgRow<'_>, user: &Uuid) -> Result<Self, tavern_db::Error> {
        let not_summary = T::try_from_row(row, user).await?;
        Ok(Self::from(&not_summary))
    }
}

#[cfg(feature = "tavern")]
#[async_trait]
impl<T: TryFromUuid + Summarize<T> + 'static> TryFromUuid for Summary<T> {
    async fn try_from_uuid(id: Uuid, user: &Uuid) -> Result<Self, tavern_db::Error> {
        let not_summary = T::try_from_uuid(id, user).await?;
        Ok(Self::from(&not_summary))
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
