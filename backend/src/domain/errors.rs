#[derive(Debug)]
pub enum DatabaseDomainError {
    ResultError(),
    ConnectionError(),
}
