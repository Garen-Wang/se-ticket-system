-- Your SQL goes here
alter table assist_info add column receiver_id integer references employee_info (id);
alter table assist_info rename column employee_id to submitter_id;