use mysql::Params;

pub trait BaseModel {

    /// get_table_name 表名
    fn get_table_name(&self) -> &str;

    /// get_alias 默认别名
    fn get_alias(&self) -> &str;

    /// get_fields_count 字段数量
    fn get_fields_count(&self) -> u16;

    /// get_field_sql 获取字段列 sql
    fn get_field_sql(&self, alias: &str) -> String;

    /// get_field_sql_not_pk 获取字段列 sql 不包括主键
    fn get_field_sql_not_pk(&self, alias: &str) -> String;

    /// get_params_insert 获取基础增加语句的参数与 sql 占位
    fn get_params_insert(&self) -> (Params, String, String);

    /// get_params_update_by_pk 获取基础更新语句的参数与 sql 占位
    fn get_params_update_by_pk(&self) -> (Params, String, String);

    /// set_pk 设置主键
    fn set_pk(&mut self, pk: u64);

    /// set_created_at 设置创建时间
    fn set_created_at(&mut self, at: u64);

    /// set_updated_at 设置最后更新时间
    fn set_updated_at(&mut self, at: u64);

}
