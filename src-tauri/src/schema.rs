// @generated automatically by Diesel CLI.

diesel::table! {
    file_tags (rowid) {
        rowid -> Integer,
        file_id -> Integer,
        tag_id -> Integer,
    }
}

diesel::table! {
    files (id) {
        id -> Integer,
        path -> Text,
        name -> Text,
    }
}

diesel::table! {
    tags (id) {
        id -> Integer,
        tag -> Text,
    }
}

diesel::joinable!(file_tags -> files (file_id));
diesel::joinable!(file_tags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    file_tags,
    files,
    tags,
);
