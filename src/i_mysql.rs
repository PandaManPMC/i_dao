use mysql::{Result, MySqlError, Error};
use::std::collections::HashMap;
use log::{trace, warn};
use std::sync::RwLock;
use std::sync::Arc;
use r2d2_mysql::{
    mysql::{OptsBuilder},
    r2d2, MySqlConnectionManager,
};

lazy_static::lazy_static! {
    static ref MYSQL_POOLS: RwLock<HashMap<String,Arc<r2d2_mysql::r2d2::Pool<MySqlConnectionManager>>>> = RwLock::new({
        let map = HashMap::new();
        map
    });
}

/// init 初始化 POOLS
/// data_source_key Pool 的 key
pub fn init(data_source_key: String, opts: OptsBuilder, max_size: u32, min_idle: u32){
    let manager = MySqlConnectionManager::new(opts);
    let pool = Arc::new(r2d2::Pool::builder().max_size(max_size).min_idle(Some(min_idle)).build(manager).unwrap());
    let _ = pool.get().expect("error getting connection from pool");

    let mut mw = MYSQL_POOLS.write().unwrap();
    mw.insert(data_source_key, pool);
}

/// get_conn 获取连接
pub fn get_conn(data_source_key: &str) -> std::result::Result<r2d2::PooledConnection<MySqlConnectionManager>, Box<dyn std::error::Error>> {
    let mr = MYSQL_POOLS.read().unwrap();
    let ds = mr.get(data_source_key);

    match ds {
        Some(pool) => {
            let conn = pool.get();
            return Ok(conn?);
        },
        None => {
            trace!("get_conn 未找到 {}", data_source_key)
        }
    }
    return Err(Box::new(Error::MySqlError(MySqlError{state: String::from("data source notfound"), message: String::from(format!("data source notfound {}", data_source_key)), code: 404})));
}


/// start_tx 启动事务
pub fn start_tx<F,R,E>(data_source_key: &str,mut closure: F) -> Result<R, E> where F:  FnMut(&mut r2d2_mysql::mysql::Transaction) -> Result<R, E>, R: std::fmt::Debug, E: std::fmt::Debug {
    let mut conn = get_conn(data_source_key).unwrap();
    let mut tx = conn.start_transaction(r2d2_mysql::mysql::TxOpts::default()).unwrap();
    let res = closure(&mut tx);
    if res.is_err() {
        let _ = tx.rollback();
        drop(conn);
        warn!("i_mysql::start_tx 事务失败 回滚! res={:?}", res);
        return res;
    }
    let _ = tx.commit();

    drop(conn);
    return res;
}

/// direct 不开启事务，直连操作数据库
pub fn direct<F,R,E>(data_source_key: &str,mut closure: F) -> Result<R, E> where F:  FnMut(&mut r2d2::PooledConnection<MySqlConnectionManager>) -> Result<R, E>, R: std::fmt::Debug, E: std::fmt::Debug {
    let mut conn = get_conn(data_source_key).unwrap();
    let res = closure(&mut conn);
    drop(conn);
    if res.is_err() {
        warn!("i_mysql::direct 失败 res={:?}", res);
        return res;
    }
    return res;
}