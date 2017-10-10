#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate mijnadres_server as adr;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use std::io::{stdin, Read};

use adr::api::Event;
use adr::schema::events;

#[derive(Insertable)]
#[table_name="events"]
struct FreshEvent {
    origin: String,
    message: String,
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn store_event(conn: &PgConnection, fresh_event: &FreshEvent) -> Event {
    use adr::schema::events;

    diesel::insert(fresh_event).into(events::table)
        .get_result(conn)
        .expect("Error saving new event")
}

fn main() {
    let connection = establish_connection();

    println!("What would you like your origin to be?");
    let mut origin = String::new();
    stdin().read_line(&mut origin).unwrap();
    let origin = &origin[..(origin.len() - 1)]; // Drop the newline character
    println!("\nOk! Let's write from {} (Press {} when finished)\n", origin, EOF);
    let mut message = String::new();
    stdin().read_to_string(&mut message).unwrap();

    let fresh_event = FreshEvent { origin: origin.to_string(), message: message.to_string() };

    let event = store_event(&connection, &fresh_event);
    println!("\nSaved event {:?}", event);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
