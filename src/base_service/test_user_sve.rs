use mysql::{Pool, Result, TxOpts, PooledConn, Transaction};
use log::{info, trace, warn};
use log::Level::Error;
use crate::{base_dao, i_source, model, foundation};
use std::collections::HashMap;
use std::any::Any;
use std::result::Result::Ok;

pub fn add(m: &mut model::test_user::TestUser) -> Result<(),Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction |  -> Result<()>  {
        return base_dao::add(tx, m);
    };
    return Ok(i_source::i_mysql::start_tx(foundation::DATA_SOURCE_KEY_DEFAULT, &mut call)?);
}

pub fn add_batch(lst: &mut Vec<&mut model::test_user::TestUser>) -> Result<(),Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction |  -> Result<()>  {
        return base_dao::add_batch(tx, lst);
    };
    return Ok(i_source::i_mysql::start_tx(foundation::DATA_SOURCE_KEY_DEFAULT, &mut call)?);
}

pub fn update_by_id(m: &mut model::test_user::TestUser) -> Result<(),Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction |  -> Result<()>  {
        return base_dao::update_by_pk(tx, m);
    };
    return Ok(i_source::i_mysql::start_tx(foundation::DATA_SOURCE_KEY_DEFAULT, &mut call)?);
}

pub fn query_list(params: &HashMap<String, Box<dyn Any>>, condition: &[foundation::dao::Condition]) -> Result<Vec<model::test_user::TestUser>, Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction |  -> Result<Vec<model::test_user::TestUser>>  {
        let result = base_dao::test_user_dao::query_list(tx, params, condition);
        return Ok(result?);
    };
    return Ok(i_source::i_mysql::start_tx(foundation::DATA_SOURCE_KEY_DEFAULT, &mut call)?);
}

pub fn find_by_id(id: u64) -> Result<Option<model::test_user::TestUser>, Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction |  -> Result<Option<model::test_user::TestUser>>  {
        let result = base_dao::test_user_dao::find_by_id(tx, id);
        return Ok(result?);
    };
    return Ok(i_source::i_mysql::start_tx(foundation::DATA_SOURCE_KEY_DEFAULT, &mut call)?);
}
