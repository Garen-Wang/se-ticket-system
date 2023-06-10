-- Your SQL goes here
insert into system_info (id, name) values (1, '酒店管理系统');

insert into approval_info (approval_level, approval_name, amount, company, system_id) values (1, '老大', 100, '有间酒店', 1);

insert into employee_info (id, name, age, position, phone, state, approval_level) values (1, '黄姥爷', 24, '姥爷', '12312341234', 0, 1);

insert into account_info (id, employee_id, account_name, password_hash, account_type, system_id) values (2, 1, '123456789', '$2b$12$xbGyri0f0.OwvuD/SBIAZOpFVI4Uk.NlwJJOB0rdWkjF8NrVUglbK', 0, 1);
