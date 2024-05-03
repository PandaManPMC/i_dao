use log::{debug, warn};
use std::time::{SystemTime};
use crate::model::BaseModel;
use r2d2_mysql::mysql::{Transaction};
use r2d2_mysql::mysql::prelude::Queryable;
use r2d2_mysql::mysql::Params;

/// add 插入单个数据，会回填 pk、created_at、updated_at
pub fn add(tx: &mut Transaction, m: &mut impl BaseModel)  -> Result<(), Box<dyn std::error::Error>> {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    m.set_created_at(now);
    m.set_updated_at(now);
    let fields = m.get_params_insert();

    let stmt = format!("INSERT INTO {} ({}) VALUE ({})", m.get_table_name(), fields.1, fields.2);
    debug!("dao::add sql={}", stmt);
    let result = tx.exec_drop(stmt.clone(), fields.0);

    if result.is_err() {
        warn!("dao::add 失败！ res={:?} sql={:?}", result, stmt.clone());
        return Ok(result?);
    }

    m.set_pk(tx.last_insert_id().unwrap());
    return Ok(());
}

/// add_batch 插入批量数据（需自行控制数量，最优在 500 条内），不会回填 pk（因为我不知道怎么获取），会回填created_at、updated_at
pub fn add_batch(tx: &mut Transaction, lst: &mut Vec<&mut impl BaseModel>)  -> Result<(), Box<dyn std::error::Error>> {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    for val in &mut *lst {
        val.set_created_at(now);
        val.set_updated_at(now);
    }

    let m = &lst[0];
    let fields = m.get_params_insert();

    let stmt = format!("INSERT INTO {} ({}) VALUES ({})", m.get_table_name(), fields.1, fields.2);
    debug!("dao::add_batch sql={}", stmt);

    let result = tx.exec_batch(stmt.clone(),
            lst.iter().map(|m| m.get_params_insert().0)
    );

    if result.is_err() {
        warn!("dao::add_batch 失败！ res={:?} sql={:?}", result, stmt);
        return Ok(result?);
    }

    return Ok(());
}

/// update_by_pk 根据 pk 更新单条数据
pub fn update_by_pk(tx: &mut Transaction, m: &mut impl BaseModel) -> Result<() ,Box<dyn std::error::Error>> {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    m.set_updated_at(now);

    let fields = m.get_params_update_by_pk();

    let stmt = format!("UPDATE {} SET {} WHERE {}", m.get_table_name(), fields.1, fields.2);
    debug!("dao::update_by_pk sql={}", stmt);
    let stmt = tx.prep(stmt)
        .unwrap();

    let result = tx.exec_drop(&stmt, fields.0);
    if result.is_err() {
        warn!("dao::update_by_id 失败！ res={:?} sql={:?}", result, stmt);
        return Ok(result?);
    }

    return Ok(());
}

/// update_by_sql 根据 sql 执行更新
pub fn update_by_sql(tx: &mut Transaction, sql: String, params: Params) -> Result<() ,Box<dyn std::error::Error>> {
    debug!("dao::update_by_sql sql={}", sql);
    let stmt = tx.prep(sql.clone())
        .unwrap();

    let result = tx.exec_drop(&stmt, params);
    if result.is_err() {
        warn!("dao::update_by_sql 失败！ res={:?} sql={:?}", result, sql);
        return Ok(result?);
    }

    return Ok(());
}