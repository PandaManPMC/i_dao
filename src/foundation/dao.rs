use mysql::Value;
use crate::foundation;

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
                    where_sql = " AND created_at >= ?".to_string()
                } else {
                    where_sql = " created_at >= ?".to_string()
                }
                params.push(create_time_begin.into());
            },
            foundation::dao::Condition::CreateTimeEnd(create_time_end) => {
                if "" != where_sql {
                    where_sql = " AND created_at < ?".to_string()
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