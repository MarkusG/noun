table! {
    note (id) {
        id -> Int4,
        thing_id -> Int4,
        created -> Timestamp,
        content -> Nullable<Text>,
    }
}

table! {
    thing (id) {
        id -> Int4,
        name -> Text,
        created -> Timestamp,
        tags -> Nullable<Array<Text>>,
    }
}

joinable!(note -> thing (thing_id));

allow_tables_to_appear_in_same_query!(
    note,
    thing,
);
