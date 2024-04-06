
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TestUserWallet{
    pub id: u64,    // search钱包编号
    pub created_at: u64,    // 创建时间
    pub updated_at: u64,    // 最后更新
    pub id_test_user: u64,  // search会员编号
    pub coin_symbol: String,  // 代币符号
    pub state: u8,  // thing状态:1@正常;2@冻结;3@锁定
    pub balance: String,  // 余额
}

/// FIELDS 字段数组
pub const FIELDS:[&str;7] = ["id", "created_at", "updated_at", "id_test_user", "coin_symbol", "state", "balance"];

/// ALIAS 默认别名
pub const ALIAS:&str = "testUserWallet";


