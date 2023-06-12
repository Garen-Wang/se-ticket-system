-- Your SQL goes here
alter table employee_info add column system_id integer not null references system_info (id);