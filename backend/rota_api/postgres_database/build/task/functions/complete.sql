-- Created: 05/06/2022
-- Modified: 05/06/2022
-- Author: Philip Cox
-- Description: rotates a task

CREATE OR REPLACE FUNCTION "task"."complete"(p_id integer)
 RETURNS boolean
 LANGUAGE plpgsql
AS $function$
    declare
        v_next_person integer;
        v_current_person integer;
        v_rotating boolean;
        v_times_completed integer;
	begin
        IF EXISTS (SELECT 1 FROM "task"."task" WHERE "id" = p_id) THEN
            SELECT "rotating" FROM "task"."task" WHERE "id" = p_id INTO v_rotating;
            SELECT "assigned_person" FROM "task"."task" WHERE "id" = p_id INTO v_current_person;
            v_next_person := v_current_person + 1;

            SELECT "times_completed" FROM "task"."stats" WHERE "task_id" = p_id AND "person_id" = v_current_person INTO v_times_completed;
            v_times_completed := v_times_completed + 1;
            UPDATE "task"."stats" SET "times_completed" = v_times_completed WHERE "task_id" = p_id AND "person_id" = v_current_person;

            IF v_rotating THEN
                IF EXISTS (SELECT 1 FROM "person"."person" WHERE "id" = v_next_person) THEN
                    UPDATE "task"."task"
                    SET "assigned_person" = v_next_person
                    WHERE "id" = p_id;
                ELSE 
                    UPDATE "task"."task"
                    SET "assigned_person" = 1
                    WHERE "id" = p_id;
                END IF;
            END IF;

            RETURN TRUE;
        ELSE
            RETURN FALSE;
        END IF;
	END;
$function$;