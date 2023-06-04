-- Your SQL goes here
alter table apply_dev_info drop constraint apply_dev_info_pkey;
alter table apply_dev_info drop column id;
alter table apply_dev_info add primary key (ticket_id, operation_id);