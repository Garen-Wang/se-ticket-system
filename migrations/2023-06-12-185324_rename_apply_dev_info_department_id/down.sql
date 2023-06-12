-- This file should undo anything in `up.sql`

alter table apply_dev_info rename column department_id to operation_id;