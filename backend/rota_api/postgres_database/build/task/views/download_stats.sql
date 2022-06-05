-- Created: 05/06/2022
-- Modified: 05/06/2022
-- Author: Philip Cox
-- Description: deletes a task

CREATE OR REPLACE VIEW "task"."download_stats" AS
    SELECT 
        pp."name" AS "person_name",
        tt."name" AS "task_name",
        ts."times_completed"
    FROM "task"."stats" ts
        INNER JOIN "task"."task" tt 
        ON ts."task_id" = tt."id"
        INNER JOIN "person"."person" pp
        ON ts."person_id" = pp."id";