mod error;
mod pb;
mod types;
mod utils;
use chrono::{DateTime, NaiveDateTime, Utc};
use core::fmt;
pub use error::Error;
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
