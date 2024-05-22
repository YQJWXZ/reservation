use async_trait::async_trait;
use sqlx::postgres::types::PgRange;

use crate::{ReservationError, ReservationId, ReservationManager, Rsvp};

#[async_trait]
impl Rsvp for ReservationManager {
    async fn reserve(&self, rsvp: abi::Reservation) -> Result<abi::Reservation, ReservationError> {
        if rsvp.start.is_none() || rsvp.end.is_none() {
            return Err(ReservationError::InvalidTime);
        }
        let timespan = PgRange::new(rsvp.start_time, rsvp.end_time);
        // generate a insert sql for the reservation
        let sql = "INSERT INTO reservations (user_id, resource_id, timespan, note, status ) VALUES ($1, $2, $3, $4, $5) RETURNING id";
        // execute the sql
        let id = sqlx::query!(
            sql,
            rsvp.user_id,
            rsvp.resource_id,
            rsvp.timespan,
            rsvp.note,
            rsvp.status
        )
        .fetch_one(&self.pool)
        .await?
        .id;
        // return the reservation
        Ok(abi::Reservation { id, ..rsvp })
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
