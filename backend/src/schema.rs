// @generated automatically by Diesel CLI.

diesel::table! {
    collection_dependencies (id) {
        id -> Integer,
        parent_id -> Integer,
        child_id -> Integer,
    }
}

diesel::table! {
    collections (id) {
        id -> Integer,
        deezer_id -> Text,
        name -> Text,
        url -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    collection_dependencies,
    collections,
);
