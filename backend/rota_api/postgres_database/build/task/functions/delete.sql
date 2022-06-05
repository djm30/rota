-- Created: 05/06/2022
-- Modified: 05/06/2022
-- Author: Philip Cox
-- Description: deletes a task

CREATE OR REPLACE FUNCTION "task"."delete"(p_id integer)
 RETURNS boolean
 LANGUAGE plpgsql
AS $function$
	begin
        IF EXISTS (SELECT 1 FROM "task"."task" WHERE "id" = p_id) THEN 
            DELETE FROM "task"."task" WHERE "id" = p_id;
            DELETE FROM "task"."stats" WHERE "task_id" = p_id;
			RETURN TRUE;
        ELSE
            RETURN FALSE;
        END IF;
	END;
$function$;