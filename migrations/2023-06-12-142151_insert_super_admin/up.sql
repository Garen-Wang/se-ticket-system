-- Your SQL goes here

insert into system_info (id, name) values (1, '超级系统');

insert into employee_info (id, name, age, position, phone, state, system_id) values
(
    1,
    '超级管理员',
    24,
    '管理员',
    '15814901579',
    0,
    1
);

insert into account_info (id, employee_id, account_name, password_hash, account_type) values 
(
    1,
    1,
    'admin',
    '$2b$12$OLb9deFd.41lt08tkpEsNuCwpGcdnzX.fNKCrcsefajLLCsuDEmwW',
    0
);

update system_info set admin_account_id = 1 where id = 1;