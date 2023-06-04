-- Your SQL goes here
alter table approved_info drop constraint approved_info_pkey;
alter table approved_info drop column id;
alter table approved_info add primary key (ticket_id, approval_id);