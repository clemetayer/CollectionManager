// @generated automatically by Diesel CLI.

diesel::table! {
    artists (id) {
        id -> Integer,
        name -> Text,
    }
}

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
        name -> Text,
    }
}

diesel::table! {
    tracks (id) {
        id -> Integer,
        isrc -> Text,
        title -> Text,
    }
}

diesel::table! {
    tracks_from_artist (id) {
        id -> Integer,
        track_id -> Integer,
        artist_id -> Integer,
    }
}

diesel::table! {
    tracks_in_collection (id) {
        id -> Integer,
        track_id -> Integer,
        collection_id -> Integer,
    }
}

diesel::joinable!(tracks_from_artist -> artists (artist_id));
diesel::joinable!(tracks_from_artist -> tracks (track_id));
diesel::joinable!(tracks_in_collection -> collections (collection_id));
diesel::joinable!(tracks_in_collection -> tracks (track_id));

diesel::allow_tables_to_appear_in_same_query!(
    artists,
    collection_dependencies,
    collections,
    tracks,
    tracks_from_artist,
    tracks_in_collection,
);
