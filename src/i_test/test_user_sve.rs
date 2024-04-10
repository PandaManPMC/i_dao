use mysql::{Result};
use crate::{dao, i_mysql, sql, i_test::test_user, i_test::test_user_dao, i_test};
use std::collections::HashMap;
use std::any::Any;
use std::result::Result::Ok;
use r2d2_mysql::mysql::Transaction;

pub fn add(m: &mut test_user::TestUser) -> Result<(), Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction | -> Result<(), Box<dyn std::error::Error>>  {
        return dao::add(tx, m);
    };
    return Ok(i_mysql::start_tx(&i_test::get_data_source_key(), &mut call)?);
}

pub fn add_batch(lst: &mut Vec<&mut test_user::TestUser>) -> Result<(),Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction |  -> Result<(), Box<dyn std::error::Error>>  {
        return dao::add_batch(tx, lst);
    };
    return Ok(i_mysql::start_tx(&i_test::get_data_source_key(), &mut call)?);
}

pub fn update_by_id(m: &mut test_user::TestUser) -> Result<(),Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction |  -> Result<(), Box<dyn std::error::Error>>  {
        return dao::update_by_pk(tx, m);
    };
    return Ok(i_mysql::start_tx(&i_test::get_data_source_key(), &mut call)?);
}

pub fn query_list(params: &HashMap<String, Box<dyn Any>>, condition: &[sql::Condition]) -> Result<Vec<test_user::TestUser>, Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction |  -> Result<Vec<test_user::TestUser>, Box<dyn std::error::Error>>  {
        let result = test_user_dao::query_list(tx, params, condition);
        return Ok(result?);
    };
    return Ok(i_mysql::start_tx(&i_test::get_data_source_key(), &mut call)?);
}

pub fn find_by_id(id: u64) -> Result<Option<test_user::TestUser>, Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction |  -> Result<Option<test_user::TestUser>, Box<dyn std::error::Error>>  {
        let result = test_user_dao::find_by_id(tx, id);
        return Ok(result?);
    };
    return Ok(i_mysql::start_tx(&i_test::get_data_source_key(), &mut call)?);
}
