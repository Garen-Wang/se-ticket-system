// @generated automatically by Diesel CLI.

diesel::table! {
    account_info (id) {
        id -> Int4,
        employee_id -> Int4,
        #[max_length = 50]
        account_name -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        account_type -> Int2,
    }
}

diesel::table! {
    apply_dev_info (ticket_id, operation_id) {
        ticket_id -> Int4,
        operation_id -> Int4,
    }
}

diesel::table! {
    approval_info (id) {
        approval_level -> Int4,
        #[max_length = 100]
        approval_name -> Varchar,
        amount -> Int4,
        #[max_length = 50]
        company -> Nullable<Varchar>,
        system_id -> Int4,
        id -> Int4,
    }
}

diesel::table! {
    approved_info (ticket_id, approval_id) {
        ticket_id -> Int4,
        approval_id -> Int4,
    }
}

diesel::table! {
    assist_info (ticket_id, employee_id) {
        ticket_id -> Int4,
        state -> Int2,
        employee_id -> Int4,
    }
}

diesel::table! {
    employee_info (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        age -> Int4,
        #[max_length = 100]
        position -> Nullable<Varchar>,
        #[max_length = 50]
        phone -> Bpchar,
        state -> Int2,
        approval_id -> Nullable<Int4>,
        system_id -> Int4,
    }
}

diesel::table! {
    employee_operation_info (e_id, o_id) {
        e_id -> Int4,
        o_id -> Int4,
    }
}

diesel::table! {
    fund_list (id) {
        id -> Int4,
        ticket_id -> Int4,
        #[max_length = 100]
        reason -> Varchar,
        amount -> Int4,
    }
}

diesel::table! {
    operation_info (id) {
        id -> Int4,
        #[max_length = 100]
        department_name -> Varchar,
        system_id -> Int4,
    }
}

diesel::table! {
    system_info (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        admin_account_id -> Nullable<Int4>,
    }
}

diesel::table! {
    ticket_info (id) {
        id -> Int4,
        creator_id -> Int4,
        approval_id -> Nullable<Int4>,
        last_approver_id -> Nullable<Int4>,
        #[max_length = 100]
        title -> Varchar,
        amount -> Int4,
        #[max_length = 500]
        reason -> Varchar,
        state -> Int2,
        #[max_length = 255]
        image -> Nullable<Varchar>,
        #[max_length = 500]
        address -> Varchar,
        created_time -> Timestamp,
        updated_time -> Timestamp,
        expired_type -> Int2,
        system_id -> Int4,
    }
}

diesel::joinable!(account_info -> employee_info (employee_id));
diesel::joinable!(apply_dev_info -> operation_info (operation_id));
diesel::joinable!(apply_dev_info -> ticket_info (ticket_id));
diesel::joinable!(approval_info -> system_info (system_id));
diesel::joinable!(approved_info -> approval_info (approval_id));
diesel::joinable!(approved_info -> ticket_info (ticket_id));
diesel::joinable!(assist_info -> employee_info (employee_id));
diesel::joinable!(assist_info -> ticket_info (ticket_id));
diesel::joinable!(employee_info -> approval_info (approval_id));
diesel::joinable!(employee_info -> system_info (system_id));
diesel::joinable!(employee_operation_info -> employee_info (e_id));
diesel::joinable!(employee_operation_info -> operation_info (o_id));
diesel::joinable!(fund_list -> ticket_info (ticket_id));
diesel::joinable!(operation_info -> system_info (system_id));
diesel::joinable!(ticket_info -> approval_info (approval_id));
diesel::joinable!(ticket_info -> system_info (system_id));

diesel::allow_tables_to_appear_in_same_query!(
    account_info,
    apply_dev_info,
    approval_info,
    approved_info,
    assist_info,
    employee_info,
    employee_operation_info,
    fund_list,
    operation_info,
    system_info,
    ticket_info,
);
