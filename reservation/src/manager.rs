use crate::{ReservationId, ReservationManager, Rsvp};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{postgres::types::PgRange, PgPool, Row};

#[async_trait]
impl Rsvp for ReservationManager {
    async fn reserve(&self, mut rsvp: abi::Reservation) -> Result<abi::Reservation, abi::Error> {
        rsvp.validate()?;

        let status = abi::ReservationStatus::from_i32(rsvp.status)
            .unwrap_or(abi::ReservationStatus::Pending);

        let timespan: PgRange<DateTime<Utc>> = rsvp.get_timespan().into();
        // generate a insert sql for the reservation
        // execute the sql
        let id = sqlx::query(
          "INSERT INTO rsvp.reservations (user_id, resource_id, timespan, note, status ) VALUES ($1, $2, $3, $4, $5::rsvp.reservation_status) RETURNING id"
        )
        .bind(rsvp.user_id.clone())
        .bind(rsvp.resource_id.clone())
        .bind(timespan)
        .bind(rsvp.note.clone())
        .bind(status.to_string())
        .fetch_one(&self.pool)
        .await?
        .get(0);

        rsvp.id = id;
        // return the reservation
        Ok(rsvp)
    }

    async fn change_status(&self, _id: ReservationId) -> Result<abi::Reservation, abi::Error> {
        todo!()
    }

    async fn update_note(
        &self,
        _id: ReservationId,
        _note: String,
    ) -> Result<abi::Reservation, abi::Error> {
        todo!()
    }

    async fn get(&self, _id: ReservationId) -> Result<abi::Reservation, abi::Error> {
        todo!()
    }

    async fn delete(&self, _id: ReservationId) -> Result<(), abi::Error> {
        todo!()
    }

    async fn query(
        &self,
        _query: abi::ReservationQuery,
    ) -> Result<Vec<abi::Reservation>, abi::Error> {
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
    use super::*;
    use chrono::{DateTime, FixedOffset};

    use crate::{ReservationManager, Rsvp};

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn reserve_should_work_for_valid_window() {
        let manager = ReservationManager::new(migrated_pool.clone());
        let start: DateTime<FixedOffset> = "2023-12-25T15:00:00-0700".parse().unwrap();
        let end: DateTime<FixedOffset> = "2023-12-28T12:00:00-0700".parse().unwrap();
        let rsvp = abi::Reservation::new_pending(
            "xiaoyiid",
            "ocean-view-room-713",
            start,
            end,
            "I'll arrive at 3pm. Please help me to upgrade to executive room if possible.",
        );

        let rsvp = manager.reserve(rsvp).await.unwrap();
        assert!(rsvp.id != 0);
    }

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn reserve_conflict_reservation_should_reject() {
        let manager = ReservationManager::new(migrated_pool.clone());
        let rsvp1 = abi::Reservation::new_pending(
            "xiaoyiid",
            "ocean-view-room-713",
            "2023-12-25T15:00:00-0700".parse().unwrap(),
            "2023-12-28T12:00:00-0700".parse().unwrap(),
            "hello.",
        );

        let rsvp2 = abi::Reservation::new_pending(
            "aliceid",
            "ocean-view-room-713",
            "2023-12-26T15:00:00-0700".parse().unwrap(),
            "2023-12-30T12:00:00-0700".parse().unwrap(),
            "hello.",
        );

        let _rsvp1 = manager.reserve(rsvp1).await.unwrap();
        let rsvp2_err = manager.reserve(rsvp2).await.unwrap();
        println!("{:?}", rsvp2_err);
    }
}
