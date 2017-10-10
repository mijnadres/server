extern crate diesel;
extern crate dotenv;
extern crate mijnadres_server as adr;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use adr::api::Event;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    use adr::schema::events::dsl::*;

    let connection = establish_connection();
    let results = events
        .limit(5)
        .load::<Event>(&connection)
        .expect("Error loading events");

    println!("Displaying {} events", results.len());
    for event in results {
        println!("{:?}", event);
    }}
