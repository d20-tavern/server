use warp::Filter;

#[tokio::main]
async fn main() {
    let server = tavern_server::setup_server()
        .with(
            warp::cors()
                .allow_credentials(true)
                .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allow_header("Content-Type")
                .allow_any_origin()
        );

    let addr = tavern_server::config::config().address.clone();
    let port = tavern_server::config::config().port;

    warp::serve(server).run((addr, port)).await;
}
