pub const TICKET_STATE_OPEN: i16 = 0; // 没人接
pub const TICKET_STATE_ASSIGNED: i16 = 1; // 有人接
pub const TICKET_STATE_CLOSED: i16 = 2; // 关闭了

pub const EMPLOYEE_STATUS_AVAILABLE: i16 = 0;
pub const EMPLOYEE_STATUS_UNAVAILABLE: i16 = 1;

pub const ACCOUNT_TYPE_ADMIN: i16 = 0; // 系统的管理员
pub const ACCOUNT_TYPE_APPROVER: i16 = 1; // 审批人员
pub const ACCOUNT_TYPE_OPERATOR: i16 = 2; // 运维人员
pub const ACCOUNT_TYPE_VIEWER: i16 = 3; // 可以看报表的人
pub const ACCOUNT_TYPE_APPLICANT: i16 = 4; // 申请工单的人

pub const APPROVAL_ID_ADMIN: i32 = 0; // 特殊的 approval id，看到就是管理员
