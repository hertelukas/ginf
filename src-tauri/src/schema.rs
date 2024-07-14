// @generated automatically by Diesel CLI.

diesel::table! {
    fileTags (rowid) {
        rowid -> Integer,
        file_id -> Nullable<Integer>,
        tag_id -> Nullable<Integer>,
    }
}

diesel::table! {
    files (id) {
        id -> Nullable<Integer>,
        path -> Text,
        name -> Text,
    }
}

diesel::table! {
    tags (id) {
        id -> Nullable<Integer>,
        tag -> Text,
    }
}

diesel::joinable!(fileTags -> files (file_id));
diesel::joinable!(fileTags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    fileTags,
    files,
    tags,
);
