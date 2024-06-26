#[derive(thiserror::Error, Debug)]
pub enum Error {
    // #[error("Invalid reservation id")]
    // InvalidReservationId,
    // #[error("Invalid reservation status")]
    // InvalidReservationStatus,
    // #[error("Invalid reservation date")]
    // InvalidReservationDate,
    #[error("Database error")]
    DbError(sqlx::Error),
    #[error("Invalid start or end time for the reservation")]
    InvalidTime,

    #[error("{0}")]
    ConflictReservation(String),
    #[error("No reservation found by the given condition")]
    NotFound,
    #[error("Invalid user id: {0}")]
    InvalidUserId(String),

    #[error("Invalid resource id: {0}")]
    InvalidResourceId(String),
    #[error("unknown error")]
    Unknown,
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::Database(e) => {
                let err = e.downcast_ref::<sqlx::postgres::PgDatabaseError>();
                match (err.code(), err.schema(), err.table()) {
                    ("23P01", Some("rsvp"), Some("reservations")) => {
                        Error::ConflictReservation(err.detail().unwrap().parse().unwrap())
                    }
                    _ => Error::DbError(sqlx::Error::Database(e)),
                }
            }

            sqlx::Error::RowNotFound => Error::NotFound,

            _ => Error::DbError(e),
        }
    }
}

// TODO: write a parser
// "Key (resource_id, timespan)=(ocean-view-room-713, [\"2022-12-26 22:00:00+00\",\"2022-12-30 19:00:00+00\")) conflicts with existing key (resource_id, timespan)=(ocean-view-room-713, [\"2022-12-25 22:00:00+00\",\"2022-12-28 19:00:00+00\"))."

// pub struct ReservationConflictInfo {
//     a: ReservationWindow,
//     b: ReservationWindow,
// }

// pub struct ReservationWindow {
//     rid: String,
//     start: DateTime<Utc>,
//     end: DateTime<Utc>,
// }
