-- Created: 05/06/2022
-- Modified: 05/06/2022
-- Author: Philip Cox
-- Description: the stats related to each task

-- Table: "task"."stats"

-- DROP TABLE "task"."task";

CREATE TABLE IF NOT EXISTS "task"."stats"
(
    "task_id" integer NOT NULL,
    "person_id" integer NOT NULL,
    "times_completed" integer NOT NULL
);