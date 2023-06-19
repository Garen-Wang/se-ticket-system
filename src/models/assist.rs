use crate::models::employee::Employee;
use crate::{models::department::Department, schema::assist_employee_info};
use diesel::{prelude::*, query_dsl::methods::FilterDsl};

use crate::{
    error::AppError,
    schema::{assist_department_info, assist_info},
    utils::constant::{TICKET_STATE_CLOSED, TICKET_STATE_OPEN},
};

#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]
#[diesel(table_name = assist_info)]
pub struct Assist {
    pub ticket_id: i32,    // 原工单
    pub state: i16,        // 0: 没人接，1：有人接，2：关闭了
    pub submitter_id: i32, // 接原工单，提交协助工单的人
    pub id: i32,
    pub receiver_id: Option<i32>, // 接协助工单的人
}

#[derive(Insertable)]
#[diesel(table_name = assist_info)]
pub struct InsertAssist {
    pub ticket_id: i32,
    pub submitter_id: i32,
}

impl Assist {
    pub fn create(
        conn: &mut PgConnection,
        insert_assist: InsertAssist,
    ) -> Result<Assist, AppError> {
        let assist = diesel::insert_into(assist_info::table)
            .values(insert_assist)
            .get_result(conn)?;
        Ok(assist)
    }

    pub fn get_by_id(conn: &mut PgConnection, id: i32) -> Result<Self, AppError> {
        let assist = assist_info::table.find(id).get_result(conn)?;
        Ok(assist)
    }

    pub fn mget_current_by_receiver(
        conn: &mut PgConnection,
        receiver_id: i32,
    ) -> Result<Vec<Assist>, AppError> {
        let assists = FilterDsl::filter(
            assist_info::table,
            assist_info::receiver_id
                .eq(receiver_id)
                .and(assist_info::state.eq(TICKET_STATE_OPEN)),
        )
        .get_results(conn)?;
        Ok(assists)
    }

    pub fn mget_history_by_receiver(
        conn: &mut PgConnection,
        receiver: i32,
    ) -> Result<Vec<Assist>, AppError> {
        use crate::schema::assist_info::dsl::*;
        let assists = diesel::QueryDsl::filter(
            assist_info,
            state.eq(TICKET_STATE_CLOSED).and(receiver_id.eq(receiver)),
        )
        .get_results::<Assist>(conn)?;
        Ok(assists)
    }
}

#[derive(Debug, Clone, Identifiable, Selectable, Queryable, Associations)]
#[diesel(belongs_to(Assist))]
#[diesel(belongs_to(Department))]
#[diesel(table_name = assist_department_info)]
#[diesel(primary_key(assist_id, department_id))]
pub struct AssistWithDepartments {
    pub id: i32,
    pub assist_id: i32,
    pub department_id: i32,
    pub total_num: i32,
    pub current_num: i32,
    pub state: i16,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = assist_department_info)]
pub struct InsertAssistWithDepartments {
    pub assist_id: i32,
    pub department_id: i32,
    pub total_num: i32,
    pub current_num: i32,
    pub state: i16,
}

impl AssistWithDepartments {
    pub fn create(
        conn: &mut PgConnection,
        assist_id: i32,
        department_id: i32,
        total_num: i32,
    ) -> Result<Self, AppError> {
        let a = diesel::insert_into(assist_department_info::table)
            .values(InsertAssistWithDepartments {
                assist_id,
                department_id,
                total_num,
                current_num: 0,
                state: TICKET_STATE_OPEN,
            })
            .get_result::<Self>(conn)?;
        Ok(a)
    }

    pub fn mget_assist_id_by_department(
        conn: &mut PgConnection,
        department_id: i32,
    ) -> Result<Vec<i32>, AppError> {
        let ids = diesel::QueryDsl::filter(
            assist_department_info::table,
            assist_department_info::department_id.eq(department_id),
        )
        .select(assist_department_info::assist_id)
        .get_results::<i32>(conn)?;
        Ok(ids)
    }

    pub fn add_person(
        conn: &mut PgConnection,
        assist_id: i32,
        department_id: i32,
    ) -> Result<Self, AppError> {
        let a = diesel::update(assist_department_info::table)
            .filter(
                assist_department_info::assist_id
                    .eq(assist_id)
                    .and(assist_department_info::department_id.eq(department_id)),
            )
            .set(assist_department_info::current_num.eq(assist_department_info::current_num + 1))
            .get_result(conn)?;
        Ok(a)
    }
}

#[derive(Debug, Clone, Identifiable, Selectable, Queryable, Associations)]
#[diesel(belongs_to(Assist))]
#[diesel(belongs_to(Employee))]
#[diesel(table_name = assist_employee_info)]
#[diesel(primary_key(assist_id, employee_id))]
pub struct AssistWithEmployees {
    pub id: i32,
    pub assist_id: i32,
    pub employee_id: i32,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = assist_employee_info)]
pub struct InsertAssistWithEmployees {
    pub assist_id: i32,
    pub employee_id: i32,
}

impl AssistWithEmployees {
    pub fn create(
        conn: &mut PgConnection,
        assist_id: i32,
        employee_id: i32,
    ) -> Result<Self, AppError> {
        let a: Self = diesel::insert_into(assist_employee_info::table)
            .values(InsertAssistWithEmployees {
                assist_id,
                employee_id,
            })
            .get_result(conn)?;
        Ok(a)
    }

    pub fn mget_assist_id_by_involver(
        conn: &mut PgConnection,
        involver_id: i32,
    ) -> Result<Vec<i32>, AppError> {
        let a: Vec<i32> = QueryDsl::filter(
            assist_employee_info::table,
            assist_employee_info::employee_id.eq(involver_id),
        )
        .select(assist_employee_info::assist_id)
        .get_results(conn)?;
        Ok(a)
    }

    pub fn mget_participant_by_assist_id(
        conn: &mut PgConnection,
        assist_id: i32,
    ) -> Result<Vec<String>, AppError> {
        let a: Vec<i32> = FilterDsl::filter(
            assist_employee_info::table,
            assist_employee_info::assist_id.eq(assist_id),
        )
        .select(assist_employee_info::employee_id)
        .get_results(conn)?;
        let mut ret = vec![];
        for id in a.into_iter() {
            let employee = Employee::get_by_id(conn, id)?;
            ret.push(employee.name);
        }
        Ok(ret)
    }
}
