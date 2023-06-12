-- Your SQL goes here

alter table employee_operation_info drop constraint employee_operation_info_pkey;
alter table employee_operation_info rename column e_id to employee_id;
alter table employee_operation_info rename column o_id to department_id;
alter table employee_operation_info add unique (employee_id, department_id);
alter table employee_operation_info add id serial;
alter table employee_operation_info add constraint employee_operation_info_pkey primary key (id);

alter table approved_info drop constraint approved_info_pkey;
alter table approved_info add unique (ticket_id, approval_id);
alter table approved_info add id serial;
alter table approved_info add constraint approved_info_pkey primary key (id);

alter table apply_dev_info drop constraint apply_dev_info_pkey;
alter table apply_dev_info add unique (ticket_id, department_id);
alter table apply_dev_info add id serial;
alter table apply_dev_info add constraint apply_dev_info_pkey primary key (id);

alter table assist_info drop constraint assist_info_pkey;
alter table assist_info add column department_id integer not null;
alter table assist_info add column amount integer not null;
alter table assist_info add id serial;
alter table assist_info add constraint assist_info_pkey primary key (id);