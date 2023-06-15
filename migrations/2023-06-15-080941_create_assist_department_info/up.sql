-- Your SQL goes here
create table assist_department_info (
    id serial primary key,
    assist_id integer not null references assist_info (id),
    department_id integer not null references operation_info (id),
    total_num integer not null,
    current_num integer not null,
    state smallint not null default 0
);