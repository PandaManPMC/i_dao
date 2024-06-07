use r2d2_mysql::mysql::Transaction;
use crate::{dao};
use crate::library_test::test_user;
use crate::tok::i_mysql;

pub async fn add(m: &mut test_user::TestUser) -> mysql::Result<(), Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction | -> mysql::Result<(), Box<dyn std::error::Error>>  {
        return dao::add(tx, m);
    };
    return Ok(i_mysql::start_tx("mysql_db1", &mut call).await?);
}
