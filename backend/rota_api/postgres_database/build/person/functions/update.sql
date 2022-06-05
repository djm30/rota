-- Created: 05/06/2022
-- Modified: 05/06/2022
-- Author: Philip Cox
-- Description: updates a person

CREATE OR REPLACE FUNCTION "person"."update"(p_id integer, p_name character varying)
 RETURNS boolean
 LANGUAGE plpgsql
AS $function$
	begin
        IF EXISTS (SELECT 1 FROM "person"."person" WHERE "name" = p_name) THEN 
			UPDATE "person"."person" SET "name" = p_name WHERE "id" = p_id; 
            RETURN TRUE;
        ELSE
            RETURN FALSE;
        END IF;
	END;
$function$;