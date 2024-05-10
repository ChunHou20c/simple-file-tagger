// @generated automatically by Diesel CLI.

diesel::table! {
    file_records (id) {
        id -> Integer,
        filename -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tags (id) {
        id -> Integer,
        tag_name -> Text,
        description -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    file_records,
    tags,
);
