-- Your SQL goes here
create table assist_employee_info(
    id serial primary key,
    assist_id integer not null references assist_info (id),
    department_id integer not null references operation_info (id),
    employee_id integer not null references employee_info (id)
);