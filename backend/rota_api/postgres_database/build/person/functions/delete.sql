-- Created: 04/06/2022
-- Modified: 05/06/2022
-- Author: Philip Cox
-- Description: deletes a person

CREATE OR REPLACE FUNCTION "person"."delete"(p_id integer)
 RETURNS boolean
 LANGUAGE plpgsql
AS $function$
	begin
        IF EXISTS (SELECT 1 FROM "person"."person" WHERE "id" = p_id) THEN 
            DELETE FROM "person"."person" WHERE "id" = p_id;
            DELETE FROM "task"."stats" WHERE "person_id" = p_id;
			RETURN TRUE;
        ELSE
            RETURN FALSE;
        END IF;
	END;
$function$;