use catalog_cluster::AppEnvironment;
use diesel::PgConnection;

fn main() {
    println!("Hello, my friend!");
    let environment = AppEnvironment::from(std::env::var);
    let _connection =
        PgConnection::try_from(environment).expect("Could not connect to the database");
    println!("We connected to the database!");
}
