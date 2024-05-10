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
    tag_records (id) {
        id -> Integer,
        file_id -> Integer,
        tag_id -> Integer,
    }
}

diesel::table! {
    tags (id) {
        id -> Integer,
        tag_name -> Text,
        description -> Nullable<Text>,
    }
}

diesel::joinable!(tag_records -> file_records (file_id));
diesel::joinable!(tag_records -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    file_records,
    tag_records,
    tags,
);
