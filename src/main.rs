use std::time::SystemTime;
use crate::base_service::test_user_sve::add;

mod model;
mod base_dao;
mod base_service;
mod i_source;
mod foundation;
mod i_util;

use env_logger::Env;
use log::{debug, info, trace, warn};
use std::collections::HashMap;
use std::any::Any;
use bson::*;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();
    debug!("Hello, world!");

    base_service::set_date_source_key(String::from("mysql_db1"));
    debug!("{:?}", base_service::get_data_source_key());

    i_source::i_mysql::init(base_service::get_data_source_key(), "mysql://root:123456@localhost:3306/test_rs");
    let conn = i_source::i_mysql::get_conn(&base_service::get_data_source_key());

    trace!("{:?}", conn);

    // test_add_batch();

    // test_add();
    test_find();
    // test_update();
    // test_query_list();

    // let t_sql = model::test_user::get_field_sql_not_pk("hello");
    // info!("{}", t_sql);

    // let t_sql2 = model::test_user::get_field_sql_update("hello");
    // info!("{}", t_sql2);
}

fn test_add_batch() {
    let mut lst: Vec<&mut model::test_user::TestUser> = Vec::new();
    let mut binding = model::test_user::TestUser::new("xcy 0409 01 2".to_string(), 1);
    lst.push(&mut binding);
    let mut binding2 = model::test_user::TestUser::new("xcy 0409 03 2".to_string(), 1);
    lst.push(&mut binding2);
    let mut binding3 = model::test_user::TestUser::new("xcy 0409 02 2".to_string(), 1);
    lst.push(&mut binding3);
    let res = base_service::test_user_sve::add_batch(&mut lst);
    debug!("{:?}", res);
    debug!("{:?}", lst);
}

fn test_query_list(){
    let mut params:HashMap<String, Box<dyn Any>> = HashMap::new();
    // params.insert(String::from(format!("{}state", foundation::dao::GT)), Box::new(1));
    params.insert(String::from(format!("{}state", foundation::dao::GT_EQ)), Box::new(1));

    // params.insert(String::from("user_name"), Box::new(String::from("XINYI_Doge")));
    params.insert(String::from(format!("{}id", foundation::dao::GT)), Box::new(1u64));

    let page_index = foundation::dao::Condition::PageIndex(1);
    let page_size = foundation::dao::Condition::PageSize(3);
    let asc = foundation::dao::Condition::OrderByAESOrDESC(1);


    let result = base_service::test_user_sve::query_list(&params, &[page_index, page_size, asc, ]);
    if result.is_err(){
        warn!("出现异常 {:?}", result);
        return;
    }
    let res = result.unwrap();
    for i in &res {
        debug!(
            "id = {}, created_at = {}, updated_at = {}, user_name = {}, state = {}",
            i.id, i.created_at, i.updated_at, i.user_name, i.state
        );

        let redacted_bson = bson::to_bson(&i).unwrap();
        debug!("{}", redacted_bson);
    }
}

fn test_update() {
    debug!("----------- test_update --------------------");
    let id: u64 = 50;

    let res = base_service::test_user_sve::find_by_id(id);
    debug!("{:?}", res);
    if res.is_err(){
        warn!("test_find 出现异常 {:?}", res);
        return;
    }

    let t = res.unwrap();
    debug!("{:?}", t);
    match t {
        Some(mut user) => {
            debug!("找到用户 user={:?}", user);
            // user.user_name = String::from("XINYI_Doge");
            user.state = 2;
            let u_res = base_service::test_user_sve::update_by_id(&mut user);
            if u_res.is_err() {
                warn!("出现异常 {:?}", u_res);
                return;
            }

            debug!("更新后的 user={:?}", user)
        },
        None => {
            debug!("未找到用户id={}", id);
        }
    }

}

fn test_find(){
    debug!("----------- test_find --------------------");
    let id: u64 = 52;

    let res = base_service::test_user_sve::find_by_id(id);
    debug!("{:?}", res);
    if res.is_err(){
        warn!("test_find 出现异常 {:?}", res);
        return;
    }

    let t = res.unwrap();
    debug!("{:?}", t);
    match t {
        Some(user) => {
            debug!("找到用户 user={:?}", user);
        },
        None => {
            debug!("未找到用户id={}", id);
        }
    }
}

fn test_add(){
    // let mut t1 = model::test_user::TestUser::new("xcy".to_string(), 1);

    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let mut t1 = model::test_user::TestUser::new(format!("xcy 0409 {}", now), 1);
    debug!("{:?}", t1);

    let add_res = base_service::test_user_sve::add(&mut t1);
    debug!("{:?}", add_res);
    if add_res.is_err() {
        debug!("调用 service 方法出现错误");
    }
    info!("{:?}", t1);
}

