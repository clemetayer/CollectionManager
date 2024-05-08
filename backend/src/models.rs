use super::schema::collections;
use crate::schema::collection_dependencies;
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
#[diesel(table_name = collection_dependencies)]
pub struct NewCollectionDependency<'a> {
    pub parent_id: &'a i32,
    pub child_id: &'a i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = collection_dependencies)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CollectionDependencies {
    pub parent_id: i32,
    pub child_id: i32,
}
