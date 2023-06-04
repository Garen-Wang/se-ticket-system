-- Your SQL goes here
alter table employee_operation_info drop constraint employee_operation_info_pkey;
alter table employee_operation_info drop column id;
alter table employee_operation_info add primary key (e_id, o_id);