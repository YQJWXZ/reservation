use crate::{ReservationError, ReservationId, ReservationManager, Rsvp};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{postgres::types::PgRange, PgPool, Row};

#[async_trait]
impl Rsvp for ReservationManager {
    async fn reserve(
        &self,
        mut rsvp: abi::Reservation,
    ) -> Result<abi::Reservation, ReservationError> {
        if rsvp.start.is_none() || rsvp.end.is_none() {
            return Err(ReservationError::InvalidTime);
        }

        // let status = abi::ReservationStatus::from_i32(rsvp.status)
        // .unwrap_or(abi::ReservationStatus::Pending);
        let start = abi::convert_to_utc_time(rsvp.start.as_ref().unwrap().clone());
        let end = abi::convert_to_utc_time(rsvp.end.as_ref().unwrap().clone());

        let timespan: PgRange<DateTime<Utc>> = (start..end).into();
        // generate a insert sql for the reservation
        // execute the sql
        let id = sqlx::query(
          "INSERT INTO rsvp.reservations (user_id, resource_id, timespan, note, status ) VALUES ($1, $2, $3, $4, $5) RETURNING id"
        )
        .bind(rsvp.user_id.clone())
        .bind(rsvp.resource_id.clone())
        .bind(timespan)
        .bind(rsvp.note.clone())
        .bind(rsvp.status)
        .fetch_one(&self.pool)
        .await?
        .get(0);

        rsvp.id = id;
        // return the reservation
        Ok(rsvp)
    }

    async fn change_status(
        &self,
        _id: ReservationId,
    ) -> Result<abi::Reservation, ReservationError> {
        todo!()
    }

    async fn update_note(
        &self,
        _id: ReservationId,
        _note: String,
    ) -> Result<abi::Reservation, ReservationError> {
        todo!()
    }

    async fn get(&self, _id: ReservationId) -> Result<abi::Reservation, ReservationError> {
        todo!()
    }

    async fn delete(&self, _id: ReservationId) -> Result<(), ReservationError> {
        todo!()
    }

    async fn query(
        &self,
        _query: abi::ReservationQuery,
    ) -> Result<Vec<abi::Reservation>, ReservationError> {
        todo!()
    }
}

impl ReservationManager {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use abi::convert_to_timestamp;
    use chrono::FixedOffset;

    use super::*;
    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn reserve_should_work_for_valid_window() {
        let manager = ReservationManager::new(migrated_pool.clone());
        let start: DateTime<FixedOffset> = "2023-12-25T15:00:00-0700"
        let end: DateTime<FixedOffset> = "2023-12-28T12:00:00-0700"
        let rsvp = abi::Reservation {
            id: "".to_string(),
            user_id: "xiaoyiid".to_string(),
            resource_id: "ocean-view-room-713".to_string(),
            start: convert_to_timestamp().into(),
            end: convert_to_timestamp(Utc::now() + Duration::days(3)).into(),
            note: "I'll arrive at 3pm. Please help me to upgrade to executive room if possible."
                .to_string(),
            status: abi::ReservationStatus::Pending as i32,
        };

        let rsvp = manager.reserve(rsvp).await.unwrap();
        assert!(rsvp.id != "");
    }
}