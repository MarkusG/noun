extern crate noun;
extern crate diesel;

use noun::*;
use models::*;
use diesel::prelude::*;

fn main() {
    use noun::schema::place::dsl::*;

    // Get our connection to the database
    let connection = establish_connection();

    // Insert a new place
    // add_place(&connection,
    //           None,
    //           None,
    //           None,
    //           Some("Test Place"),
    //           Some(vec!["Foo", "Bar", "Baz"]),
    //           "A cool place");

    
    // Query the place table
    let results = place.load::<Place>(&connection)
        .expect("Error loading places");

    println!("Displaying {} places", results.len());
    for p in results {
        println!("{:#?}\n\n", p);
    }
}
