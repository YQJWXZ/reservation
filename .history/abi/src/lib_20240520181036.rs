mod pb;

pub use pb::*;

fn convert_to_utc_time() -> DateTime<Utc> {
    let start = rsvp.start.unwrap();
    let start = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(start.seconds, start.nativeseconds as _),
        Utc,
    );
}
