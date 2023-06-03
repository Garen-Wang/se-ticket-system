-- Your SQL goes here
create table account_info (
    id serial primary key,
    employee_id serial references employee_info (id),
    account_name varchar(50) not null ,
    password_hash varchar(255) not null ,
    account_type smallint default 0 not null
);

comment on column account_info.id is '帐号ID';
comment on column account_info.employee_id is '对应员工ID';
comment on column account_info.account_name is '帐号名称';
comment on column account_info.password_hash is '帐号密码摘要';
comment on column account_info.account_type is '帐号类型，0申请者，1审批者';
