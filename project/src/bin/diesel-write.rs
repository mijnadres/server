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

use adr::api::ContactRecord;
use adr::schema::contactrecords;

#[derive(Insertable)]
#[table_name="contactrecords"]
struct FreshContactRecord {
    name: String,
    phone: String,
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn store_event(conn: &PgConnection, fresh_contact_record: &FreshContactRecord) -> ContactRecord {
    use adr::schema::contactrecords;

    diesel::insert(fresh_contact_record).into(contactrecords::table)
        .get_result(conn)
        .expect("Error saving new event")
}

fn main() {
    let connection = establish_connection();

    println!("What would you like your name to be?");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = &name[..(name.len() - 1)]; // Drop the newline character
    println!("\nOk! What is {} phone number? (Press {} when finished)\n", name, EOF);
    let mut phone = String::new();
    stdin().read_to_string(&mut phone).unwrap();
    let phone = &phone[..(phone.len() - 1)]; // Drop the newline character

    let fresh_contact_record = FreshContactRecord { name: name.to_string(), phone: phone.to_string() };

    let contact_record = store_event(&connection, &fresh_contact_record);
    println!("\nSaved contact record {:?}", contact_record);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
