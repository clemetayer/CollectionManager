mod common;
pub mod models;
pub mod schema;

use common::common::get_env_variable;
use diesel::prelude::*;

pub fn establish_connection() -> Result<SqliteConnection, ConnectionError> {
    let database_url: String = get_env_variable("DATABASE_URL");
    SqliteConnection::establish(&(database_url.as_str()))
}
