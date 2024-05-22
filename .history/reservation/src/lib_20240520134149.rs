mod error;
pub use error::ReservationError;

pub type ReservationId = String;
pub trait Rsvp {
    /// make a reservation
    fn reserve(&self, rsvp: abi::Reservation) -> Result<abi::Reservation, ReservationError>;
    /// change reservation status (if current status is pending, change it to confirmed)
    fn change_status(&self, id: ReservationId) -> Result<abi::Reservation, ReservationError>;
    /// update note
    
}
