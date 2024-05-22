mod pb;

use chrono::{DateTime, NaiveDateTime, Utc};
pub use pb::*;
use prost_types::Timestamp;

fn convert_to_utc_time(ts: Timestamp) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(ts.seconds, ts.nativeseconds as _),
        Utc,
    )
}
