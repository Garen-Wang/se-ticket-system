-- Your SQL goes here
create table operation_info (
    id serial primary key,
    department_name varchar(100) not null
);
comment on column operation_info.id is '部门ID';
comment on column operation_info.department_name is '部门名字';