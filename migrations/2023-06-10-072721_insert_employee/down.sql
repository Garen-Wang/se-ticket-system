-- This file should undo anything in `up.sql`

delete from employee_info where id = 1;
delete from approval_info where approval_level = 1;
delete from system_info where id = 1;