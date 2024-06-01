#[derive(thiserror::Error, Debug)]
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

    #[error("{0}")]
    ConflictReservatio(String),
    #[error("Invalid user id: {0}")]
    InvalidUserId(String),

    #[error("Invalid resource id: {0}")]
    InvalidResourceId(String),
    #[error("unknown error")]
    Unknown,
}