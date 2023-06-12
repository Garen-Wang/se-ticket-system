-- Your SQL goes here
alter table system_info add column admin_account_id integer references account_info (id);
