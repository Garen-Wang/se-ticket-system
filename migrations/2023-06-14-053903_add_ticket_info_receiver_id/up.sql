-- Your SQL goes here
alter table ticket_info add column receiver_id integer default null references employee_info (id);