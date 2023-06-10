-- This file should undo anything in `up.sql`
alter table approval_info drop column system_id;
alter table ticket_info drop column system_id;
alter table operation_info drop column system_id;
alter table account_info drop column system_id;