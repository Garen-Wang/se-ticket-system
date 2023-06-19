pub const TICKET_STATE_UNAPPROVED: i16 = 0; // 未审批
pub const TICKET_STATE_APPROVING: i16 = 1; // 审批中
pub const TICKET_STATE_OPEN: i16 = 2; // 审批完，还没人接
pub const TICKET_STATE_ASSIGNED: i16 = 3; // 有人接
pub const TICKET_STATE_CLOSED: i16 = 4; // 关闭了
pub const TICKET_STATE_REJECTED: i16 = 5; // 审批驳回

pub const EMPLOYEE_STATUS_AVAILABLE: i16 = 0;
pub const EMPLOYEE_STATUS_UNAVAILABLE: i16 = 1;

pub const ACCOUNT_TYPE_ADMIN: i16 = 0; // 系统的管理员
pub const ACCOUNT_TYPE_APPROVER: i16 = 1; // 审批人员
pub const ACCOUNT_TYPE_OPERATOR: i16 = 2; // 运维人员
pub const ACCOUNT_TYPE_VIEWER: i16 = 3; // 可以看报表的人
pub const ACCOUNT_TYPE_APPLICANT: i16 = 4; // 申请工单的人

pub const APPROVAL_ID_ADMIN: i32 = 0; // 特殊的 approval id，看到就是管理员

pub const SEX_FEMALE: i16 = 0;
pub const SEX_MALE: i16 = 1;

pub const APPROVE_RESULT_APPROVED: i16 = 1;
pub const APPROVE_RESULT_REJECTED: i16 = 0;

pub const IMAGE_URL_PREFIX: &str = "http://8.134.67.143:7878";
