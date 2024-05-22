use crate::{ReservationError, ReservationId, ReservationManager, Rsvp};
use async_trait::async_trait;
use chrono::{Date, DateTime, NaiveDateTime, Utc};
use sqlx::postgres::types::PgRange;

#[async_trait]
impl Rsvp for ReservationManager {
    async fn reserve(&self, rsvp: abi::Reservation) -> Result<abi::Reservation, ReservationError> {
        if rsvp.start.is_none() || rsvp.end.is_none() {
            return Err(ReservationError::InvalidTime);
        }

        let status = abi::ReservationStatus::from_i32(rsvp.status)
            .unwrap_or(abi::ReservationStatus::Pending);
        let start = abi::convert_to_utc_time(rsvp.start.unwrap());
        let end = abi::convert_to_utc_time(rsvp.end.unwrap());

        let timespan: PgRange<DateTime<Utc>> = (start..end).into();
        // generate a insert sql for the reservation
        // execute the sql
        let id = sqlx::query!(
          "INSERT INTO reservation (user_id, resource_id, timespan, note, status ) VALUES ($1, $2, $3, $4, $5) RETURNING id",
            rsvp.user_id,
            rsvp.resource_id,
            timespan,
            rsvp.note,
            rsvp.status
        )
        .bind(rsvp.user_id)
        .bind(rsvp.resource_id)
        .bind(timespan)
        .bind(rsvp.note)
        .bind(rsvp.status)
        .fetch_one(&self.pool)
        .await?
        .get(0);
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
