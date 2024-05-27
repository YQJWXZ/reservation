mod error;
mod pb;
mod types;
mod utils;

use core::fmt;
pub use error::Error;
pub use pb::*;

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
