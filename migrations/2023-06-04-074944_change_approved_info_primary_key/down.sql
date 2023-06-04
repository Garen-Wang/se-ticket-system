-- This file should undo anything in `up.sql`
alter table approved_info drop constraint approved_info_pkey;
alter table approved_info add id serial;
alter table approved_info add constraint approved_info_pkey primary key (id);