pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use self::models::{Place, NewPlace};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn add_place<'a>(conn: &PgConnection,
                     lat: Option<f64>,
                     long: Option<f64>,
                     address: Option<&'a str>,
                     name: Option<&'a str>,
                     tags: Option<Vec<&'a str>>,
                     description: &'a str) -> Place {
    use schema::place;

    let new_place = NewPlace {
        lat: lat,
        long: long,
        address: address,
        name: name,
        tags: tags,
        description: description
    };

    diesel::insert_into(place::table)
            .values(&new_place)
        .get_result(conn)
        .expect("Error inserting thing")
}
