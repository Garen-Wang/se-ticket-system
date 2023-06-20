use crate::{error::AppError, schema::system_info};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Selectable, Identifiable, Queryable)]
#[diesel(table_name = system_info)]
pub struct System {
    pub id: i32,
    pub name: String,
    pub admin_account_id: Option<i32>,
    pub initialized: i16, // 1: initialized, 0: uninitialized
}

#[derive(Insertable)]
#[diesel(table_name = system_info)]
pub struct InsertSystem<'a> {
    pub name: &'a str,
}

impl System {
    pub fn create(conn: &mut PgConnection, name: &str) -> Result<System, AppError> {
        let ret: System = diesel::insert_into(system_info::table)
            .values(InsertSystem { name })
            .get_result(conn)?;
        Ok(ret)
    }

    pub fn get_by_id(conn: &mut PgConnection, id: i32) -> Result<System, AppError> {
        let system: System = system_info::table.find(id).first(conn)?;
        Ok(system)
    }

    pub fn set_admin_account_id(
        conn: &mut PgConnection,
        id: i32,
        account_id: i32,
    ) -> Result<System, AppError> {
        let system: System = diesel::update(system_info::table.filter(system_info::id.eq(id)))
            .set(system_info::admin_account_id.eq(account_id))
            .get_result(conn)?;
        Ok(system)
    }

    pub fn set_initialized(
        conn: &mut PgConnection,
        id: i32,
        initialized: i16,
    ) -> Result<System, AppError> {
        let system = diesel::update(system_info::table.find(id))
            .set(system_info::initialized.eq(initialized))
            .get_result(conn)?;
        Ok(system)
    }
}
