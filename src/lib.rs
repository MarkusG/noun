pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::env;
use std::{error::Error, fmt};

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;

use self::models::{Place, NewPlace};

#[derive(Debug)]
pub enum NounError {
    DbError
}

impl Error for NounError {}

impl fmt::Display for NounError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NounError::DbError => write!(f, "Database error")
        }
    }
}

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
                     description: &'a str) -> Result<Place, Box<dyn Error>> {
    use schema::place;

    // Construct a NewPlace struct to prepare for insertion
    let new_place = NewPlace {
        lat: lat,
        long: long,
        address: address,
        name: name,
        tags: tags,
        description: description
    };

    // Execute the insertion, saving our newly inserted row
    let result = diesel::insert_into(place::table)
        .values(&new_place)
        .get_result(conn);

    // If there was a database error, wrap it up in an Err(Box<dyn Error>)
    match result {
        Ok(p) => Ok(p),
        Err(e) => Err(Box::new(e))
    }
}
