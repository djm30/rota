-- Created: 03/06/2022
-- Modified: 05/06/2022
-- Author: Philip Cox
-- Description: creates a task

CREATE OR REPLACE FUNCTION "task"."create"(p_name character varying, p_description character varying, p_assigned_person integer, p_rotating boolean)
 RETURNS integer
 LANGUAGE plpgsql
AS $function$
	declare 
		v_task_id integer;
        v_person_ids integer[];
        v_person_id integer;
	begin
        IF EXISTS (SELECT 1 FROM "task"."task" WHERE "name" = p_name) THEN 
			RETURN -1;
        ELSE
            SELECT public.download_next_id('task','task') INTO v_task_id;
            INSERT INTO "task"."task"("id", "name", "description", "assigned_person", "rotating") VALUES(v_task_id, p_name, p_description, p_assigned_person, p_rotating);
            
            v_person_ids := ARRAY(SELECT "id" FROM "person"."person");

            FOREACH v_person_id IN ARRAY v_person_ids LOOP
                INSERT INTO "task"."stats"("task_id", "person_id", "times_completed") VALUES(v_task_id, v_person_id, 0);
            END LOOP;

            RETURN v_task_id;
        END IF;
	END;
$function$;