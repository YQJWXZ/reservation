use async_trait::async_trait;

use crate::{ReservationManager, Rsvp};

#[async_trait]
impl Rsvp for ReservationManager {}
