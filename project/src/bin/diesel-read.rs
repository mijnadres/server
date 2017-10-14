extern crate diesel;
extern crate dotenv;
extern crate mijnadres_server as adr;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use adr::api::ContactRecord;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    use adr::schema::contactrecords::dsl::*;

    let connection = establish_connection();
    let results = contactrecords
        .limit(5)
        .load::<ContactRecord>(&connection)
        .expect("Error loading events");

    println!("Displaying {} contact records", results.len());
    for contactrecord in results {
        println!("{:?}", contactrecord);
    }
}
