use super::schema::collections;
use crate::schema::tracks;
use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = collections)]
pub struct NewCollection<'a> {
    pub name: &'a str,
    pub url: &'a str,
    pub deezer_id: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = collections)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Collection {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub deezer_id: String,
}

#[derive(Insertable)]
#[diesel(table_name = tracks)]
pub struct NewTrack<'a> {
    pub title: &'a str,
    pub url: &'a str,
    pub deezer_id: &'a str,
    pub artist: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = tracks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Track {
    pub id: i32,
    pub deezer_id: String,
    pub title: String,
    pub url: String,
    pub artist: String,
}
