pub mod test_user_dao;

use log::{debug, info, warn};
use mysql::*;
use mysql::prelude::*;
use std::time::{SystemTime, Duration};
use crate::{foundation, model};
use std::fmt;

/// add 插入单个数据，会回填 pk、created_at、updated_at
pub fn add(tx: &mut Transaction, m: &mut impl foundation::model::BaseModel)  -> Result<()> {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    m.set_created_at(now);
    m.set_updated_at(now);
    let fields = m.get_params_insert();

    let stmt = format!("INSERT INTO {} ({}) VALUE ({})", m.get_table_name(), fields.1, fields.2);
    debug!("b_d::add sql={}", stmt);
    let result = tx.exec_drop(stmt, fields.0);

    if result.is_err() {
        warn!("b_d::add 失败！ res={:?}", result);
        return result;
    }

    m.set_pk(tx.last_insert_id().unwrap());
    return Ok(());
}

/// add_batch 插入批量数据（需自行控制数量，最优在 500 条内），不会回填 pk（因为我不知道怎么获取），会回填created_at、updated_at
pub fn add_batch(tx: &mut Transaction, lst: &mut Vec<&mut impl foundation::model::BaseModel>)  -> Result<()> {

    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    for val in &mut *lst {
        val.set_created_at(now);
        val.set_updated_at(now);
    }

    let m = &lst[0];
    let fields = m.get_params_insert();

    let stmt = format!("INSERT INTO {} ({}) VALUES ({})", m.get_table_name(), fields.1, fields.2);
    debug!("b_d::add_batch sql={}", stmt);

    let result = tx.exec_batch(stmt,
            lst.iter().map(|m| m.get_params_insert().0)
    );

    if result.is_err() {
        warn!("b_d::add_batch 失败！ res={:?}", result);
        return result;
    }

    // m.set_pk(tx.last_insert_id().unwrap());
    return Ok(());
}

/// update_by_pk 根据 pk 更新单条数据
pub fn update_by_pk(tx: &mut Transaction, m: &mut impl foundation::model::BaseModel) -> Result<()> {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    m.set_updated_at(now);

    let fields = m.get_params_update_by_pk();

    let stmt = format!("UPDATE {} SET {} WHERE {}", m.get_table_name(), fields.1, fields.2);
    debug!("b_d::add sql={}", stmt);
    let stmt = tx.prep(stmt)
        .unwrap();

    let result = tx.exec_drop(&stmt, fields.0);
    if result.is_err() {
        warn!("b_d::test_user_dao::update_by_id 失败！ res={:?}", result);
        return result;
    }

    return Ok(());
}