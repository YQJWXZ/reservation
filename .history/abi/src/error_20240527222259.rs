use sqlx::{error, Error};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    // #[error("Invalid reservation id")]
    // InvalidReservationId,
    // #[error("Invalid reservation status")]
    // InvalidReservationStatus,
    // #[error("Invalid reservation date")]
    // InvalidReservationDate,
    // #[error("Database error")]
    // DbError(#[from] error::Error),
    #[error("Invalid start or end time for the reservation")]
    InvalidTime,
    #[error("Invalid user id: {0}")]
    InvalidUserId(String),

    #[error("Invalid resource id: {0}")]
    InvalidResourceId(String),
    #[error("unknown error")]
    Unknown,
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::Database(e) => {
                let err: &PgDatabaseError = e.downcast_ref();
                match (err.code(), err.schema(), err.table()) {
                    ("23P01", Some("rsvp"), Some("reservations")) => {
                        Error::ConflictReservation(err.detail().unwrap().parse().unwrap())
                    }
                    _ => Error::DbError(sqlx::Error::Database(e)),
                }
            }
            sqlx::Error::RowNotFound => Error::NotFound,
            _ => Error::DbError(e),
        }
    }
}
