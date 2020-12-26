extern crate noun;
extern crate diesel;

use noun::*;
use models::*;
use diesel::prelude::*;

fn main() {
    use noun::schema::thing::dsl::*;

    // Get our connection to the database
    let connection = establish_connection();

    // Insert a new thing
    // add_thing(&connection, "Foobar", Some(vec!["baz", "adz"]));
    
    // Query the thing table
    let results = thing.load::<Thing>(&connection)
        .expect("Error loading things");

    println!("Displaying {} things", results.len());
    for t in results {
        println!("{}\n{:?}\n{:?}", t.name, t.tags, t.created);
    }
}
