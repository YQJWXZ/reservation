use async_trait::async_trait;

use crate::{ReservationError, ReservationId, ReservationManager, Rsvp};

#[async_trait]
impl Rsvp for ReservationManager {
    async fn reserve(&self, rsvp: abi::Reservation) -> Result<abi::Reservation, ReservationError> {
        // generate a insert sql for the reservation
        let sql = "INSERT INTO reservations (user_id, resource_id, start_time, end_time, note, status ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)";
    }

    async fn change_status(&self, id: ReservationId) -> Result<abi::Reservation, ReservationError> {
        todo!()
    }

    async fn update_note(
        &self,
        id: ReservationId,
        note: String,
    ) -> Result<abi::Reservation, ReservationError> {
        todo!()
    }

    async fn get(&self, id: ReservationId) -> Result<abi::Reservation, ReservationError> {}

    async fn delete(&self, id: ReservationId) -> Result<(), ReservationError> {
        todo!()
    }

    async fn query(
        &self,
        query: abi::ReservationQuery,
    ) -> Result<Vec<abi::Reservation>, ReservationError> {
        todo!()
    }
}
