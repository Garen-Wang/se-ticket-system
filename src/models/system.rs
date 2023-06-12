use crate::{error::AppError, schema::system_info};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Selectable, Identifiable, Queryable)]
#[diesel(table_name = system_info)]
pub struct System {
    pub id: i32,
    pub name: String,
    pub admin_account_id: Option<i32>,
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
}
