use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum InternalError {
    PoolError(sqlx::Error),
    DatabaseError(sqlx::Error)
}

impl Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl Error for InternalError {}
