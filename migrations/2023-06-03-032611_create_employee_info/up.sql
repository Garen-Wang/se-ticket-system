-- Your SQL goes here
create table employee_info (
    id serial primary key,
    name varchar(50) not null,
    age int not null,
    position varchar(100) null,
    phone char(50) not null,
    state smallint default 0 not null,
    approval_level int null references approval_info (approval_level)
);
comment on column employee_info.id is '员工ID';
comment on column employee_info.name is '员工名字';
comment on column employee_info.age is '员工年龄';
comment on column employee_info.position is '员工职位';
comment on column employee_info.phone is '员工电话';
comment on column employee_info.state is '员工状态，1忙碌，0空闲';
comment on column employee_info.approval_level is '员工对应的审批等级';