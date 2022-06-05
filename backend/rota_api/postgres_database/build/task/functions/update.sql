-- Created: 05/06/2022
-- Modified: 05/06/2022
-- Author: Philip Cox
-- Description: updates a task

CREATE OR REPLACE FUNCTION "task"."update"(p_id integer, p_name character varying, p_description character varying, p_assigned_person integer, p_rotating boolean)
 RETURNS boolean
 LANGUAGE plpgsql
AS $function$
	begin
        IF EXISTS (SELECT 1 FROM "task"."task" WHERE "id" = p_id) THEN 
			UPDATE "task"."task"
            SET "name" = p_name, "description" = p_description, "assigned_person" = p_assigned_person, "rotating" = p_rotating
            WHERE "id" = p_id;
            RETURN TRUE;
        ELSE
            RETURN FALSE;
        END IF;
	END;
$function$;