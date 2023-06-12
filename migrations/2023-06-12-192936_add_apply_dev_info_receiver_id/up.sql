-- Your SQL goes here
alter table apply_dev_info add column receiver_id integer references employee_info (id);