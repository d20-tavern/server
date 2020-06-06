#[tavern_derive::db_test]
async fn init_database_succeeds() {
    // db_test macro automatically inits once
}

#[tavern_derive::db_test]
async fn double_database_init_succeeds() {
    tavern_server::db::init()
        .await
        .expect("second database init failed");
}
