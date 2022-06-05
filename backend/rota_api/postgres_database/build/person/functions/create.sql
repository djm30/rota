-- Created: 03/06/2022
-- Modified: 05/06/2022
-- Author: Philip Cox
-- Description: creates a person

CREATE OR REPLACE FUNCTION "person"."create"(p_name character varying)
 RETURNS integer
 LANGUAGE plpgsql
AS $function$
	declare 
		v_person_id integer;
        v_task_ids integer[];
        v_task_id integer;
	begin
        IF EXISTS (SELECT 1 FROM "person"."person" WHERE "name" = p_name) THEN 
			RETURN -1;
        ELSE
            SELECT public.download_next_id('person','person') INTO v_person_id;
            INSERT INTO "person"."person"("id", "name") VALUES(v_person_id, p_name);

            v_task_ids := ARRAY(SELECT "id" FROM "task"."task");

            FOREACH v_task_id IN ARRAY v_task_ids LOOP 
                IF NOT EXISTS (SELECT 1 FROM "task"."stats" WHERE "task_id" = v_task_id AND "person_id" = v_person_id) THEN
                    INSERT INTO "task"."stats"("task_id", "person_id", "times_completed") VALUES(v_task_id, v_person_id, 0);
                END IF;
            END LOOP;

            RETURN v_person_id;
        END IF;
	END;
$function$;