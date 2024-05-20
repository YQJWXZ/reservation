
-- if user_id is null, find all reservations within during for the resource
-- if resource_id is null, find all reservations within during for the user
-- if both are null, find all reservations within during
-- if both set, find all reservations within during for the resource and user
CREATE OR REPLACE FUNCTION rsvp.query(uid text, rid text, during: TSTZRANGE) RETURNS TABLE rsvp.reservations 
AS $$ 
BEGIN
  -- if user_id is null, find all reservations within during for the resource
    IF uid IS NULL AND rid IS NOT NULL THEN
        RETURN QUERY SELECT * FROM rsvp.reservations WHERE resource_id = rid AND during @> start_time;
END;
$$ LANGUAGE plpgsql;