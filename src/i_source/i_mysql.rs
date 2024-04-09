use mysql::{Pool, Result, TxOpts, PooledConn, MySqlError, Error};
use::std::collections::HashMap;
use lazy_static::lazy_static;
use log::{debug, trace, warn};
use mysql::serde_json::value;
use std::sync::RwLock;
use std::fmt::Display;
use std::error;
use crate::{foundation, i_source};
use std::any::Any;
use mysql::*;

lazy_static::lazy_static! {
    static ref MYSQL_POOLS: RwLock<HashMap<String,Pool>> = RwLock::new({
        let map = HashMap::new();
        map
    });
}

/// init 初始化 POOLS
/// data_source_key Pool 的 key
/// url = "mysql://root:123456@localhost:3306/test_rs";
pub fn init(data_source_key: String, url: &str){
    let pool = Pool::new(url).unwrap();
    let _ = pool.get_conn().unwrap();

    let mut mw = MYSQL_POOLS.write().unwrap();
    mw.insert(data_source_key, pool);
}

/// get_conn 获取连接
pub fn get_conn(data_source_key: &str) -> std::result::Result<PooledConn, Box<dyn std::error::Error>> {
    let mr = MYSQL_POOLS.read().unwrap();
    let ds = mr.get(data_source_key);

    match ds {
        Some(pool) => {
            trace!("{:?}", pool);
            // PooledConn
            return Ok(pool.get_conn()?);
        },
        None => {
            trace!("get_conn 未找到 {}", data_source_key)
        }
    }
    return Err(Box::new(Error::MySqlError(MySqlError{state: String::from("data source notfound"), message: String::from(format!("data source notfound {}", data_source_key)), code: 404})));
}


/// start_tx 启动事务
pub fn start_tx<F,R>(data_source_key: &str,mut closure: F) -> Result<R> where F:  FnMut(&mut Transaction) -> Result<R>, R: std::fmt::Debug {
    let mut conn = get_conn(data_source_key);
    let mut binding = conn.unwrap();
    let mut tx = binding.start_transaction(TxOpts::default()).unwrap();
    let res = closure(&mut tx);
    if res.is_err() {
        let _ = tx.rollback();
        warn!("i_mysql::start_tx 事务失败 回滚! res={:?}", res);
        return res;
    }
    let _ = tx.commit();

    drop(binding);
    return res;
}