-- Created: 03/06/2022
-- Modified: 03/06/2022
-- Author: Philip Cox
-- Description: the tasks

-- Table: "task"."taks"

-- DROP TABLE "task"."task";

CREATE TABLE IF NOT EXISTS "task"."task"
(
    "id" integer NOT NULL,
    "name" character varying COLLATE pg_catalog."default" NOT NULL,
    "description" character varying COLLATE pg_catalog."default" NOT NULL,
    "assigned_person" integer NOT NULL,
    "rotating" boolean NOT NULL
);