use std::any::Any;
use std::collections::HashMap;
use r2d2_mysql::mysql::Transaction;
use crate::{dao, library_test, sql};
use crate::library_test::{test_user, test_user_dao};
use crate::tok::i_mysql;

pub async fn add(m: &mut test_user::TestUser) -> mysql::Result<(), Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction | -> mysql::Result<(), Box<dyn std::error::Error>>  {
        return dao::add(tx, m);
    };
    return Ok(i_mysql::start_tx("mysql_db1", &mut call).await?);
}

pub async fn query_list_by_enum(params: &HashMap<String, sql::Params>, condition: &[sql::Condition]) -> mysql::Result<Vec<test_user::TestUser>, Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction |  -> mysql::Result<Vec<test_user::TestUser>, Box<dyn std::error::Error>>  {
        let result = test_user_dao::query_list_by_enum(tx, params, condition);
        return Ok(result?);
    };
    return Ok(i_mysql::start_tx("mysql_db1", &mut call).await?);
}