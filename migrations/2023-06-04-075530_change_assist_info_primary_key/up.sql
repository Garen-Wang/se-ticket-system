-- Your SQL goes here
alter table assist_info drop constraint assist_info_pkey;
alter table assist_info drop column id;
alter table assist_info add primary key (ticket_id, receiver_id);