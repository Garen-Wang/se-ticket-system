-- Your SQL goes here
create table ticket_info (
    id serial primary key,
    creator_id serial references employee_info (id),
    approval_level int null,
    last_approver_id int null references approval_info (approval_level),
    title varchar(100) not null,
    amount int not null,
    reason varchar(500) not null,
    state smallint default 0 not null,
    image varchar(255) null,
    address varchar(500) not null,
    created_time timestamp default CURRENT_TIMESTAMP not null,
    updated_time timestamp default CURRENT_TIMESTAMP not null,
    expired_type smallint default 0 not null
);
comment on column ticket_info.id is '工单ID';
comment on column ticket_info.creator_id is '提交工单的员工ID';
comment on column ticket_info.approval_level is '派生值，根据 amount 推测最终要到哪一级审批';
comment on column ticket_info.last_approver_id is '上一个审批人的员工ID，可能为空';
comment on column ticket_info.title is '工单标题';
comment on column ticket_info.amount is '工单金额';
comment on column ticket_info.reason is '工单理由';
comment on column ticket_info.state is '工单当前状态，0未开始，1审批中，2已审批未执行，3执行中，4执行完';
comment on column ticket_info.image is '图片路径，可为空';
comment on column ticket_info.address is '地址';
comment on column ticket_info.created_time is '工单创建时间';
comment on column ticket_info.updated_time is '工单更新时间';
comment on column ticket_info.expired_type is '工单过期状态';