-- This file should undo anything in `up.sql`
alter table assist_info drop constraint assist_info_pkey;
alter table assist_info add id serial;
alter table assist_info add constraint assist_info_pkey primary key (id);