-- Created: 20/02/2022
-- Modified: 20/02/2022
-- Author: Philip Cox
-- Description: downloads the next id for a given schema table

CREATE OR REPLACE FUNCTION "public"."download_next_id"(p_schema character varying, p_table character varying)
 RETURNS integer
 LANGUAGE plpgsql
AS $function$
	declare 
		v_next_id integer;
	begin		
		execute format('set search_path to "%s"', p_schema);
		execute format('
		select * FROM (
		    SELECT t1.id+1 AS id
		    FROM "%s"."%s" t1
		    WHERE NOT EXISTS(SELECT * FROM "%s"."%s" t2 WHERE t2.Id = t1.Id + 1 )
		    UNION 
		    SELECT 1 AS id
		    WHERE NOT EXISTS (SELECT * FROM "%s"."%s" t3 WHERE t3.Id = 1)) ot
		ORDER BY id asc limit 1', p_schema, p_table, p_schema, p_table, p_schema, p_table) into v_next_id;
        
		return v_next_id;
	END;
$function$;