-- Add down migration script here
DROP TRIGGER reservations_trigger ON rsvp.reservations;
DROP FUNCTION rsvp.reservations_trigger();