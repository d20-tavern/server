//
// db.rs
// Copyright (C) 2020 shadow53 <shadow53@shadow53.com>
// Distributed under terms of the MIT license.
//

use tavern_server::db::{self, Error};
use sqlx::Connection as _;

#[cfg(feature = "test-db")]
pub(crate) async fn database_setup() {
    db::init()
        .await
        .expect("database initialization failed");
}

#[cfg(feature = "test-db")]
pub(crate) async fn clean_database() -> Result<(), Error> {
    let conn = db::get_connection().await?;
    let mut tx = conn.begin().await
        .map_err(|err| Error::Transaction(err))?;
    sqlx::query(r"
        SELECT 'TRUNCATE ' || input_table_name || ' CASCADE;' AS truncate_query
        FROM(SELECT table_schema || '.' || table_name AS input_table_name
        FROM information_schema.tables WHERE table_schema
        NOT IN ('pg_catalog', 'information_schema') AND table_schema NOT LIKE 'pg_toast%')
        AS information; 
        ")
        .execute(&mut tx)
        .await
        .map_err(|err| Error::RunQuery(err))?;
    tx.commit().await
        .map_err(|err| Error::Transaction(err))?;
    Ok(())
}

#[cfg(feature = "test-db")]
pub(crate) async fn database_teardown() {
    clean_database()
        .await
        .expect("database cleanup failed");
}

#[tokio::test]
#[cfg(feature = "test-db")]
async fn init_database_succeeds() {
    database_teardown().await;
    // database_setup initializes the database already
    database_setup().await;
}

#[tokio::test]
#[cfg(feature = "test-db")]
async fn double_database_init_succeeds() {
    database_teardown().await;
    // database_setup initializes the database once
    database_setup().await;
    db::init().await
        .expect("second database init failed");
}
