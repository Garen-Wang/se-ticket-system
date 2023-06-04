-- This file should undo anything in `up.sql`
alter table employee_operation_info drop constraint employee_operation_info_pkey;
alter table employee_operation_info add id serial;
alter table employee_operation_info add constraint employee_operation_info_pkey primary key (id);