use sqlx::Error;

pub enum ServerError {
    DatabaseError(Error)
}
