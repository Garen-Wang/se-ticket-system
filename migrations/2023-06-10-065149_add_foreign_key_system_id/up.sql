-- Your SQL goes here
alter table approval_info add column system_id integer not null references system_info (id);
alter table ticket_info add column system_id integer not null references system_info (id);
alter table operation_info add column system_id integer not null references system_info (id);
alter table account_info add column system_id integer not null references system_info (id);
