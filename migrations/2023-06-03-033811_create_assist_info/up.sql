-- Your SQL goes here
create table assist_info (
    id serial primary key,
    ticket_id serial references ticket_info (id),
    state smallint default 0 not null,
    receiver_id serial references employee_info (id)
);
comment on column assist_info.id is 'ID没意义';
comment on column assist_info.ticket_id is '主工单ID';
comment on column assist_info.state is '当前状态';
comment on column assist_info.receiver_id is '接受人ID';
comment on table assist_info is '协助工单';