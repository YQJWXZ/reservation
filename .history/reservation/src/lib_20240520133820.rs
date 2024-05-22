mod error;
pub use error::ReservationError;
pub trait Rsvp {
    fn reserve(&self, rsvp: abi::Reservation) -> Result<abi::Reservation, abi::Error>;
}
