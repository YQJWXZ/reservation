use crate::{ReservationManager, Rsvp};

impl Rsvp for ReservationManager {
    #[doc = " make a reservation"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn reserve<'life0, 'async_trait>(
        &'life0 self,
        rsvp: abi::Reservation,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<abi::Reservation, ReservationError>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[doc = " change reservation status (if current status is pending, change it to confirmed)"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn change_status<'life0, 'async_trait>(
        &'life0 self,
        id: ReservationId,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<abi::Reservation, ReservationError>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[doc = " update note"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn update_note<'life0, 'async_trait>(
        &'life0 self,
        id: ReservationId,
        note: String,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<abi::Reservation, ReservationError>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[doc = " delete reservation"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn delete<'life0, 'async_trait>(
        &'life0 self,
        id: ReservationId,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<(), ReservationError>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[doc = " get reservation by id"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn get<'life0, 'async_trait>(
        &'life0 self,
        id: ReservationId,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<abi::Reservation, ReservationError>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[doc = " query reservations"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn query<'life0, 'async_trait>(
        &'life0 self,
        query: abi::ReservationQuery,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Vec<abi::Reservation>, ReservationError>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }
}
