-- This file should undo anything in `up.sql`
alter table assist_info drop column receiver_id;
alter table assist_info rename column submitter_id to employee_id;
