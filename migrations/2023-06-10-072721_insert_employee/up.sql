-- Your SQL goes here
insert into system_info (id, name) values (1, '酒店管理系统');

insert into approval_info (approval_level, approval_name, amount, company, system_id) values (1, '老大', 100, '有间酒店', 1);

insert into employee_info (id, name, age, position, phone, state, approval_level) values (1, '黄姥爷', 24, '姥爷', '12312341234', 0, 1);