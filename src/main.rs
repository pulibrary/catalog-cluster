use catalog_cluster::{AppEnvironment, start_http_server};
use diesel::PgConnection;
use std::env;

#[tokio::main]
async fn main() {
    let environment = AppEnvironment::from(std::env::var);
    if matches!(environment, AppEnvironment::Development) {
        let _connection =
            PgConnection::try_from(&environment).expect("Could not connect to the database");
        println!("We connected to the database!");
    }
    println!("Starting server");
    start_http_server(env::var, &environment)
        .await
        .expect("Could not start HTTP server");
}
