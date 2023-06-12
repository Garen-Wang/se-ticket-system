-- This file should undo anything in `up.sql`
alter table account_info add column system_id integer not null references system_info (id);