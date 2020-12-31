use super::schema::place;

use chrono::NaiveDateTime;

#[derive(Queryable)]
#[derive(Debug)]
pub struct Place {
    pub id: i32,
    pub lat: Option<f64>,
    pub long: Option<f64>,
    pub address: Option<String>,
    pub name: Option<String>,
    pub tags: Option<Vec<String>>,
    pub description: String,
    pub recorded: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="place"]
pub struct NewPlace<'a> {
    pub lat: Option<f64>,
    pub long: Option<f64>,
    pub address: Option<&'a str>,
    pub name: Option<&'a str>,
    pub tags: Option<Vec<&'a str>>,
    pub description: &'a str,
}
