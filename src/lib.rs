pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use self::models::{Thing, NewThing};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn add_thing<'a>(conn: &PgConnection,
                     name: &'a str,
                     tags: Option<Vec<&'a str>>) -> Thing {
    use schema::thing;

    let new_thing = NewThing {
        name: name,
        tags: tags,
    };

    diesel::insert_into(thing::table)
        .values(&new_thing)
        .get_result(conn)
        .expect("Error inserting thing")
}
