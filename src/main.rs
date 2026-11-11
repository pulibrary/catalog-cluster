use catalog_cluster::{AppEnvironment, start_http_server};
use std::env;

#[tokio::main]
async fn main() {
    let environment = AppEnvironment::from(std::env::var);
    if matches!(environment, AppEnvironment::Development) {
        let pool = environment
            .database_connection()
            .await
            .expect("Could not connect to the database");
        let _connection = pool
            .get()
            .await
            .expect("Could not get a connection from the pool");
        println!("We connected to the database!");
    }
    println!("Starting server");
    start_http_server(env::var, &environment)
        .await
        .expect("Could not start HTTP server");
}
