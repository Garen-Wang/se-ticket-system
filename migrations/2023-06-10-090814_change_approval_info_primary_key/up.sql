-- Your SQL goes here
alter table approval_info add id serial unique;

alter table employee_info drop constraint employee_info_approval_level_fkey;
alter table employee_info rename column approval_level to approval_id;
alter table employee_info add constraint employee_info_approval_id_fkey foreign key (approval_id) references approval_info (id);

alter table ticket_info drop constraint ticket_info_last_approver_id_fkey;
alter table ticket_info rename column approval_level to approval_id;
alter table ticket_info add constraint ticket_info_approval_id_fkey foreign key (approval_id) references approval_info (id);
alter table ticket_info add constraint ticket_info_last_approver_id_fkey foreign key (last_approver_id) references employee_info (id);

alter table approved_info drop constraint approved_info_approval_id_fkey;
alter table approved_info add constraint approved_info_approval_id_fkey foreign key (approval_id) references approval_info (id);

alter table approval_info drop constraint approval_info_pkey;
alter table approval_info add constraint approval_info_pk primary key (id);
