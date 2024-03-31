#[derive(Debug)]
pub enum DatabaseDomainError {
    ResultError(diesel::result::Error),
    ConnectionError(),
}
