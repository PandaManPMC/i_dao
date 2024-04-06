use log::debug;
use mysql::Value;
use crate::foundation;
use std::any::Any;

/// LT 小于
pub const LT:&str = "?<?";

/// GT 大于
pub const GT:&str = "?>?";

/// LT_EQ 小于等于
pub const LT_EQ:&str = "?<=";

/// GT_EQ 大于等于
pub const GT_EQ:&str = "?>=";

/// NO_EQ 不等于
pub const NO_EQ:&str = "!=?";



// Condition SQL 列表查询条件
#[derive(Debug)]
pub enum Condition {
    None,
    PageIndex(i64),
    PageSize(i64),
    CreateTimeBegin(u64),
    CreateTimeEnd(u64),
    OrderByField(String),
    /// OrderByAESOrDESC 1 = AES ； Default -> 2 = DESC
    OrderByAESOrDESC(u8),
}

/// sql_placeholder 生成指定数量的 sql 占位符 ?，使用 , 隔开
pub fn sql_placeholder(count: u16) -> String {
    let mut p = String::from("");
    for _ in 0..count {
        if "" != p {
            p.push(',');
        }
        p.push('?');
    }
    return p;
}

/// pot_base_condition 基础条件分拣
pub fn pot_base_condition(params: &mut Vec<Value>, condition: &[foundation::dao::Condition]) -> (String, i64, i64, String, String){
    let mut where_sql = String::from("");
    let mut page_index:i64 = 1;
    let mut page_size:i64 = 20;
    let mut order_by_sql_field = String::from("");
    let mut order_by_sql_type = String::from("DESC");

    for c in condition{
        match c {
            foundation::dao::Condition::PageIndex(a_page_index) => {
                page_index = *a_page_index
            },
            foundation::dao::Condition::PageSize(a_page_size) => {
                page_size = *a_page_size;
            },
            foundation::dao::Condition::CreateTimeBegin(create_time_begin) => {
                if "" != where_sql {
                    where_sql = format!("{} AND created_at >= ?", where_sql)
                } else {
                    where_sql = " created_at >= ?".to_string()
                }
                params.push(create_time_begin.into());
            },
            foundation::dao::Condition::CreateTimeEnd(create_time_end) => {
                if "" != where_sql {
                    where_sql = format!("{} AND created_at < ?", where_sql);
                } else {
                    where_sql = " created_at < ?".to_string()
                }
                params.push(create_time_end.into());
            },
            foundation::dao::Condition::OrderByField(order_by_field) => {
                order_by_sql_field = String::from(order_by_field);
            },
            foundation::dao::Condition::OrderByAESOrDESC(order_by_aes_or_desc) => {
                if 1 == *order_by_aes_or_desc {
                    order_by_sql_type = "ASC".to_string();
                }
            },
            _ => todo!()
        }
    }

    return (where_sql, page_index, page_size, order_by_sql_field, order_by_sql_type)
}

/// pot_params_condition 参数条件分拣
pub fn pot_params_condition(params: &mut Vec<Value>, val :&Box<dyn Any>) -> bool {
    let mut i = 0;
    while i < 11 {
        match val.downcast_ref::<String>() {
            Some(v) => {
                params.push(v.into());
                return true;
            },
            None => {}
        }
        match val.downcast_ref::<u64>() {
            Some(v) => {
                params.push(v.into());
                return true;
            },
            None => {}
        }
        match val.downcast_ref::<u8>() {
            Some(v) => {
                params.push(v.into());
                return true;
            },
            None => {}
        }
        match val.downcast_ref::<i32>() {
            Some(v) => {
                params.push(v.into());
                return true;
            },
            None => {}
        }
        match val.downcast_ref::<i64>() {
            Some(v) => {
                params.push(v.into());
                return true;
            },
            None => {}
        }
        match val.downcast_ref::<u32>() {
            Some(v) => {
                params.push(v.into());
                return true;
            },
            None => {}
        }
        match val.downcast_ref::<u16>() {
            Some(v) => {
                params.push(v.into());
                return true;
            },
            None => {}
        }
        match val.downcast_ref::<u128>() {
            Some(v) => {
                params.push(v.into());
                return true;
            },
            None => {}
        }
        match val.downcast_ref::<i128>() {
            Some(v) => {
                params.push(v.into());
                return true;
            },
            None => {}
        }
        match val.downcast_ref::<i16>() {
            Some(v) => {
                params.push(v.into());
                return true;
            },
            None => {}
        }
        match val.downcast_ref::<i8>() {
            Some(v) => {
                params.push(v.into());
                return true;
            },
            None => {}
        }
        i+=1;
    }

    debug!("pot_params_condition 参数未找到条件!");
    return false;
}


/// get_real_key_operator 获取条件键附带的运算符以及截取真实的键
pub fn get_real_key_operator(key: String) -> (String, String) {
    let mut i_key = key.clone();
    let mut operator = "=";

    if 3 < key.len() {
        let prefix = key[0..3].to_string();
        match prefix.as_str() {
            foundation::dao::LT => {
                i_key = key[3..].to_string();
                operator = "<";
            },
            foundation::dao::GT => {
                i_key = key[3..].to_string();
                operator = ">";
            },
            foundation::dao::LT_EQ => {
                i_key = key[3..].to_string();
                operator = "<=";
            },
            foundation::dao::GT_EQ => {
                i_key = key[3..].to_string();
                operator = ">=";
            },
            foundation::dao::NO_EQ => {
                i_key = key[3..].to_string();
                operator = "!=";
            },
            _ => {
                // 不是运算符
            }
        }
    }
    return (i_key, operator.to_string());
}