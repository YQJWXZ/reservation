mod pb;

use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use core::fmt;
pub use pb::*;
use prost_types::Timestamp;
pub fn convert_to_utc_time(ts: Timestamp) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(ts.seconds, ts.nanos as _),
        Utc,
    )
}

pub fn convert_to_timestamp(dt: DateTime<Utc>) -> Timestamp {
    Timestamp {
        seconds: dt.timestamp(),
        nanos: dt.timestamp_subsec_nanos() as _,
    }
}

impl fmt::Display for ReservationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReservationStatus::Pending => write!(f, "pending"),
            ReservationStatus::Confirmed => write!(f, "confirmed"),
            ReservationStatus::Blocked => write!(f, "blocked"),
            ReservationStatus::Unknown => write!(f, "unknown"),
        }
    }
}

impl Reservation {
    pub fn new_pending(
        uid: impl Into<String>,
        rid: impl Into<String>,
        start: DateTime<FixedOffset>,
        end: DateTime<FixedOffset>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            uid: uid.into(),
            rid: rid.into(),
            start: convert_to_timestamp(start),
            end: convert_to_timestamp(end),
            note: note.into(),
            status: ReservationStatus::Pending as i32,
        }
    }
}
