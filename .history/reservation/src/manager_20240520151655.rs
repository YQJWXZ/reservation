use async_trait::async_trait;

use crate::{ReservationError, ReservationId, ReservationManager, Rsvp};

#[async_trait]
impl Rsvp for ReservationManager {
    async fn reserve(&self, rsvp: abi::Reservation) -> Result<abi::Reservation, ReservationError> {
        todo!()
    }

    async fn change_status(&self, id: ReservationId) -> Result<abi::Reservation, ReservationError> {
        todo!()
    }

    async fn delete(&self, id: ReservationId) -> Result<(), ReservationError> {
        todo!()
    }

    async fn get(&self, id: ReservationId) -> Result<abi::Reservation, ReservationError> {}
}
