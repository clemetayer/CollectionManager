use super::schema::collections;
use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = collections)]
pub struct NewCollection<'a> {
    pub name: &'a str
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = collections)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Collection {
    pub id: i32,
    pub name: String,
}