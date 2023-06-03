-- Your SQL goes here
create table apply_dev_info (
    id serial primary key,
    ticket_id serial references ticket_info (id),
    operation_id serial references operation_info (id)
);
comment on column apply_dev_info.id is 'ID无意义';
comment on column apply_dev_info.ticket_id is '工单ID';
comment on column apply_dev_info.operation_id is '部门ID';
comment on table apply_dev_info is '记录工单与部门多对多关系';