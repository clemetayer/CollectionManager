use crate::domain::errors::DatabaseDomainError;

#[derive(Debug)]
pub enum HandlerError {
    HandlerDatabaseError(DatabaseDomainError),
    HandlerDeezerError(),
}
