create table employee_operation_info (
    id serial primary key,
    e_id serial references employee_info (id),
    o_id serial references operation_info (id)
);
comment on column employee_operation_info.id is '默认ID，无意义';
comment on column employee_operation_info.e_id is '员工ID';
comment on column employee_operation_info.o_id is '部门ID';
comment on table employee_operation_info is '记录员工与部门多对多关系';