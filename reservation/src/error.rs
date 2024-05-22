use sqlx::error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReservationError {
    // #[error("Invalid reservation id")]
    // InvalidReservationId,
    // #[error("Invalid reservation status")]
    // InvalidReservationStatus,
    // #[error("Invalid reservation date")]
    // InvalidReservationDate,
    #[error("Database error")]
    DbError(#[from] error::Error),
    #[error("Invalid start or end time for the reservation")]
    InvalidTime,
    #[error("unknown error")]
    Unknow,
}
