-- This file should undo anything in `up.sql`
alter table apply_dev_info drop constraint apply_dev_info_pkey;
alter table apply_dev_info add id serial;
alter table apply_dev_info add constraint apply_dev_info_pkey primary key (id);