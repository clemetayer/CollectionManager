#[derive(Debug)]
pub enum DatabaseError {
    ResultError(),
    ConnectionError(),
}
