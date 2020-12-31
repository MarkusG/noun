table! {
    place (id) {
        id -> Int4,
        lat -> Nullable<Float8>,
        long -> Nullable<Float8>,
        address -> Nullable<Text>,
        name -> Nullable<Text>,
        tags -> Nullable<Array<Text>>,
        description -> Text,
        recorded -> Timestamp,
    }
}
