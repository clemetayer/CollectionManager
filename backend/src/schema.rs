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

diesel::table! {
    tracks (id) {
        id -> Integer,
        deezer_id -> Text,
        title -> Text,
        url -> Text,
        artist -> Text,
    }
}

diesel::table! {
    tracks_in_collection (id) {
        id -> Integer,
        track_id -> Integer,
        collection_id -> Integer,
    }
}

diesel::joinable!(tracks_in_collection -> collections (collection_id));
diesel::joinable!(tracks_in_collection -> tracks (track_id));

diesel::allow_tables_to_appear_in_same_query!(
    collection_dependencies,
    collections,
    tracks,
    tracks_in_collection,
);
