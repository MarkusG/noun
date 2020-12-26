use super::schema::thing;

use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Thing {
    pub id: i32,
    pub name: String,
    pub created: NaiveDateTime,
    pub tags: Option<Vec<String>>
}

#[derive(Insertable)]
#[table_name="thing"]
pub struct NewThing<'a> {
    pub name: &'a str,
    pub tags: Option<Vec<&'a str>>
}
