-- This file should undo anything in `up.sql`

alter table assist_info drop constraint assist_info_pkey;
alter table assist_info drop column id;
alter table assist_info drop column department_id;
alter table assist_info drop column amount;
alter table assist_info add primary key (ticket_id, receiver_id);

alter table apply_dev_info drop constraint apply_dev_info_pkey;
alter table apply_dev_info drop column id;
alter table apply_dev_info add primary key (ticket_id, operation_id);

alter table approved_info drop constraint approved_info_pkey;
alter table approved_info drop column id;
alter table approved_info add primary key (ticket_id, approval_id);

alter table employee_operation_info drop constraint employee_operation_info_pkey;
alter table employee_operation_info drop column id;
alter table employee_operation_info rename column employee_id to e_id;
alter table employee_operation_info rename column department_id to o_id;
alter table employee_operation_info add primary key (e_id, o_id);