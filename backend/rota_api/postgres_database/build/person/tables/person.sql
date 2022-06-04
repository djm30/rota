-- Created: 03/06/2022
-- Modified: 03/06/2022
-- Author: Philip Cox
-- Description: the tasks

-- Table: "person"."person"

-- DROP TABLE "person"."person";

CREATE TABLE IF NOT EXISTS "person"."person"
(
    "id" integer NOT NULL,
    "name" character varying COLLATE pg_catalog."default" NOT NULL    
);