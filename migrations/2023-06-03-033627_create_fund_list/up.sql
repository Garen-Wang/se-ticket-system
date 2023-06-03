-- Your SQL goes here
create table fund_list (
    id serial primary key,
    ticket_id serial references ticket_info (id),
    reason varchar(100) not null,
    amount int not null
);
comment on column fund_list.id is 'ID没意义';
comment on column fund_list.ticket_id is '工单ID';
comment on column fund_list.reason is '申请原因';
comment on column fund_list.amount is '申请金额';
comment on table fund_list is '记录工单与预算多对多关系';