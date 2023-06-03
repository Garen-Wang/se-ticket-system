-- Your SQL goes here
create table approval_info (
    approval_level serial primary key,
    approval_name varchar(100) not null,
    amount int not null,
    company varchar(50) default null
);

comment on column approval_info.approval_level is '审批等级';
comment on column approval_info.approval_name is '审批等级名字';
comment on column approval_info.amount is '对应金额';
comment on column approval_info.company is '公司名称，默认为空';