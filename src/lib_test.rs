use r2d2_mysql::mysql::OptsBuilder;
use env_logger::Env;
use log::{debug, info, trace, warn};
use std::collections::HashMap;
use std::any::Any;
use std::time::{Duration, SystemTime};
use crate::library_test::test_user::TestUser;
use crate::library_test::test_user_sve;
use crate::{library_test, i_mysql, sql};

fn test_init() {
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();
    debug!("Hello, world!");

    library_test::set_date_source_key(String::from("mysql_db1"));
    debug!("{:?}", library_test::get_data_source_key());

    let opts = OptsBuilder::new()
        .ip_or_hostname(Some("localhost"))
        .user(Some("root"))
        .pass(Some("123456"))
        .db_name(Some("test_rs"))
        .tcp_port(3306)
        .tcp_connect_timeout(Some(Duration::from_secs(30)));

    i_mysql::init(library_test::get_data_source_key(), opts, 200, 5);
    let conn = i_mysql::get_conn(&library_test::get_data_source_key());
    trace!("{:?}", conn);
}

#[cfg(test)]
mod tests {
    use crate::library_test::test_user_sve_tok;
    use crate::tok;
    use super::*;

    #[tokio::test]
    async fn test_tokio_query_list_by_enum() {
        env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();
        debug!("Hello, world!");

        library_test::set_date_source_key(String::from("mysql_db1"));
        debug!("{:?}", library_test::get_data_source_key());

        let opts = OptsBuilder::new()
            .ip_or_hostname(Some("localhost"))
            .user(Some("root"))
            .pass(Some("123456"))
            .db_name(Some("test_rs"))
            .tcp_port(3306)
            .tcp_connect_timeout(Some(Duration::from_secs(30)));

        tok::i_mysql::init(library_test::get_data_source_key(), opts, 200, 5).await;
        let conn = i_mysql::get_conn(&library_test::get_data_source_key());
        trace!("{:?}", conn);

        let mut params:HashMap<String, sql::Params> = HashMap::new();
        params.insert(String::from(format!("{}state", sql::GT_EQ)), sql::Params::UInteger8(1));
        params.insert(String::from(format!("{}id", sql::GT)), sql::Params::UInteger64(1));

        let page_index = sql::Condition::PageIndex(1);
        let page_size = sql::Condition::PageSize(3);
        let asc = sql::Condition::OrderByAESOrDESC(1);

        let bc = [page_index, page_size, asc, ];

        let result = test_user_sve_tok::query_list_by_enum(&params, &bc).await;
        if result.is_err(){
            warn!("出现异常 {:?}", result);
            return;
        }
        let res = result.unwrap();
        info!("查询到={:?}条", res.len());
        for i in &res {
            debug!(
                "id = {}, created_at = {}, updated_at = {}, user_name = {}, state = {}",
                i.id, i.created_at, i.updated_at, i.user_name, i.state
            );
        }

        // let result = test_user_sve::query_count(&params, &bc);
        // info!("查询 query_count 数量={:?}", result)
    }
    #[tokio::test]
    async fn test_tokio(){
        env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();
        debug!("Hello, world!");

        library_test::set_date_source_key(String::from("mysql_db1"));
        debug!("{:?}", library_test::get_data_source_key());

        let opts = OptsBuilder::new()
            .ip_or_hostname(Some("localhost"))
            .user(Some("root"))
            .pass(Some("123456"))
            .db_name(Some("test_rs"))
            .tcp_port(3306)
            .tcp_connect_timeout(Some(Duration::from_secs(30)));

        tok::i_mysql::init(library_test::get_data_source_key(), opts, 200, 5).await;
        let conn = i_mysql::get_conn(&library_test::get_data_source_key());
        trace!("{:?}", conn);

        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        let mut t1 = TestUser::new(format!("xcy 0409 {}", now), 1);
        debug!("{:?}", t1);

        let add_res = library_test::test_user_sve_tok::add(&mut t1).await;
        debug!("{:?}", add_res);
        if add_res.is_err() {
            debug!("调用 service 方法出现错误");
        }
        info!("{:?}", t1);

    }

    #[test]
    fn test_add() {
        test_init();
        // let mut t1 = model::test_user::TestUser::new("xcy".to_string(), 1);

        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        let mut t1 = TestUser::new(format!("xcy 0409 {}", now), 1);
        debug!("{:?}", t1);

        let add_res = library_test::test_user_sve::add(&mut t1);
        debug!("{:?}", add_res);
        if add_res.is_err() {
            debug!("调用 service 方法出现错误");
        }
        info!("{:?}", t1);
    }

    #[test]
    fn test_find() {
        test_init();
        debug!("----------- test_find --------------------");
        let id: u64 = 71;

        let res = test_user_sve::find_by_id(id);
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

    #[test]
    fn test_query_list(){
        test_init();
        let mut params:HashMap<String, Box<dyn Any>> = HashMap::new();
        // params.insert(String::from(format!("{}state", foundation::dao::GT)), Box::new(1));
        params.insert(String::from(format!("{}state", sql::GT_EQ)), Box::new(1));

        // params.insert(String::from("user_name"), Box::new(String::from("XINYI_Doge")));
        params.insert(String::from(format!("{}id", sql::GT)), Box::new(7u64));

        let page_index = sql::Condition::PageIndex(1);
        let page_size = sql::Condition::PageSize(3);
        let asc = sql::Condition::OrderByAESOrDESC(1);

        let bc = [page_index, page_size, asc, ];

        let result = test_user_sve::query_list(&params, &bc);
        if result.is_err(){
            warn!("出现异常 {:?}", result);
            return;
        }
        let res = result.unwrap();
        info!("查询到={:?}条", res.len());
        for i in &res {
            debug!(
                "id = {}, created_at = {}, updated_at = {}, user_name = {}, state = {}",
                i.id, i.created_at, i.updated_at, i.user_name, i.state
            );
        }

        let result = test_user_sve::query_count(&params, &bc);
        info!("查询 query_count 数量={:?}", result)
    }

    #[test]
    fn test_update() {
        test_init();

        debug!("----------- test_update --------------------");
        let id: u64 = 75;

        let res = test_user_sve::find_by_id(id);
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
                let u_res = test_user_sve::update_by_id(&mut user);
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

    #[test]
    fn test_add_batch(){
        test_init();
        let mut lst: Vec<&mut TestUser> = Vec::new();
        let mut binding = TestUser::new("xcy 0409 01 7".to_string(), 1);
        lst.push(&mut binding);
        let mut binding2 = TestUser::new("xcy 0409 03 7".to_string(), 1);
        lst.push(&mut binding2);
        let mut binding3 = TestUser::new("xcy 0409 02 7".to_string(), 1);
        lst.push(&mut binding3);
        let res = test_user_sve::add_batch(&mut lst);
        debug!("{:?}", res);
        debug!("{:?}", lst);
    }

}
