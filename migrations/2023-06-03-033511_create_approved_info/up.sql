-- Your SQL goes here
create table approved_info (
    id serial primary key,
    ticket_id serial references ticket_info (id),
    approval_id serial references approval_info (approval_level)
);
comment on column approved_info.id is 'ID没意义';
comment on column approved_info.ticket_id is '工单ID';
comment on column approved_info.approval_id is '审批层级ID';
comment on table approved_info is '记录已审批工单与审批层级多对多关系';