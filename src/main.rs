use catalog_cluster::{AppEnvironment, start_http_server};
use std::env;

#[tokio::main]
async fn main() {
    let environment = AppEnvironment::from(std::env::var);
    if matches!(environment, AppEnvironment::Development) {
        let _connection = environment
            .connect()
            .await
            .expect("Could not connect to the database");
        println!("We connected to the database!");
    }
    println!("Starting server");
    start_http_server(env::var, &environment)
        .await
        .expect("Could not start HTTP server");
}
