mod pb;

pub use pb::*;
use prost_types::Timestamp;

fn convert_to_utc_time(ts: Timestamp) -> DateTime<Utc> {
    let start = rsvp.start.unwrap();
    let start = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(ts.seconds, ts.nativeseconds as _),
        Utc,
    );
}
