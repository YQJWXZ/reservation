use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReservationError {
    // #[error("Invalid reservation id")]
    // InvalidReservationId,
    // #[error("Invalid reservation status")]
    // InvalidReservationStatus,
    // #[error("Invalid reservation date")]
    // InvalidReservationDate,
    #[error("Invalid  time")]
    #[error("unknowm data store error")]
    Unknow,
}
