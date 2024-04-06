use mysql::{Pool, Result, TxOpts, PooledConn};
use log::{trace, warn};
use crate::{base_dao, i_source, model, foundation};
use std::collections::HashMap;
use std::any::Any;

pub fn add(m: &mut model::test_user::TestUser) -> Result<(),Box<dyn std::error::Error>> {
    let mut conn = i_source::i_mysql::get_conn(foundation::DATA_SOURCE_KEY_DEFAULT);
    let mut binding = conn?;
    let mut tx = binding.start_transaction(TxOpts::default()).unwrap();

    let result = base_dao::add(&mut tx, m);

    if result.is_err() {
        let _ = tx.rollback();
        warn!("b_s::test_user_sve::add 事务失败 回滚! res={:?}", result);
        return Ok(result?);
    }

    let _ = tx.commit();
    return Ok(());
}

pub fn add_batch(lst: &mut Vec<&mut model::test_user::TestUser>) -> Result<(),Box<dyn std::error::Error>> {
    let mut conn = i_source::i_mysql::get_conn(foundation::DATA_SOURCE_KEY_DEFAULT);
    let mut binding = conn?;
    let mut tx = binding.start_transaction(TxOpts::default()).unwrap();

    let result = base_dao::add_batch(&mut tx, lst);

    if result.is_err() {
        let _ = tx.rollback();
        warn!("b_s::test_user_sve::add_batch 事务失败 回滚! res={:?}", result);
        return Ok(result?);
    }

    let _ = tx.commit();
    return Ok(());
}

pub fn update_by_id(m: &mut model::test_user::TestUser) -> Result<(),Box<dyn std::error::Error>> {
    let mut conn = i_source::i_mysql::get_conn(foundation::DATA_SOURCE_KEY_DEFAULT);
    let mut binding = conn?;
    let mut tx = binding.start_transaction(TxOpts::default()).unwrap();

    let result = base_dao::update_by_pk(&mut tx, m);

    if result.is_err() {
        let _ = tx.rollback();
        warn!("b_s::test_user_sve::update_by_id 事务失败 回滚! res={:?}", result);
        return Ok(result?);
    }

    let _ = tx.commit();
    return Ok(());
}


pub fn query_list()  -> Result<Vec<model::test_user::TestUser>, Box<dyn std::error::Error>> {
    let mut conn = i_source::i_mysql::get_conn(foundation::DATA_SOURCE_KEY_DEFAULT);
    let mut binding = conn?;
    let mut tx = binding.start_transaction(TxOpts::default()).unwrap();

    let page_index = foundation::dao::Condition::PageIndex(1);
    let page_size = foundation::dao::Condition::PageSize(3);
    let asc = foundation::dao::Condition::OrderByAESOrDESC(1);

    let mut condition_field:HashMap<String, Box<dyn Any>> = HashMap::new();
    // condition_field.insert(String::from("Blue"), Box::new(10));
    // condition_field.insert(String::from("Red"), Box::new(1.44));
    // condition_field.insert(String::from("yellow"), Box::new("哈哈"));
    condition_field.insert(String::from(format!("{}state", foundation::dao::GT)), Box::new(1));
    condition_field.insert(String::from("user_name"), Box::new(String::from("XINYI_Doge")));


    let result = base_dao::test_user_dao::query_list(&mut tx, condition_field,&[page_index, page_size, asc, ]);
    if result.is_err() {
        let _ = tx.rollback();
        warn!("b_s::test_user_sve::query_list 事务失败 回滚! res={:?}", result);
        return Ok(result?);
    }
    let _ = tx.commit();
    return Ok(result?);
}

pub fn find_by_id(id: u64) -> Result<Option<model::test_user::TestUser>, Box<dyn std::error::Error>> {
    let mut conn = i_source::i_mysql::get_conn(foundation::DATA_SOURCE_KEY_DEFAULT);
    let mut binding = conn?;
    let mut tx = binding.start_transaction(TxOpts::default()).unwrap();

    let result = base_dao::test_user_dao::find_by_id(&mut tx, id);

    if result.is_err() {
        let _ = tx.rollback();
        warn!("b_s::test_user_sve::find_by_id 事务失败 回滚! res={:?}", result);
        return Ok(result?);
    }

    let _ = tx.commit();
    return Ok(result?);
}
