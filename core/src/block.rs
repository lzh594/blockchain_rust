use chrono::Utc;
use serde::{Deserialize, Serialize};

use utils::coder;

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockHeader {
    pub time: i64,
    pub tx_hash: String,
    pub pre_hash: String,
}

#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub data: String,
}

impl Block {
    // 求取区块哈希
    fn set_hash(&mut self) {
        let header = coder::my_serialize(&(self.header));
        self.hash = coder::get_hash(&header[..]);
    }
    // 区块构建函数
    pub fn new(data: String, pre_hash: String) -> Block {
        let transactions = coder::my_serialize(&data);
        let tx_hash = coder::get_hash(&transactions[..]);
        let time = Utc::now().timestamp();
        let mut block = Block {
            header: BlockHeader {
                time,
                tx_hash,
                pre_hash,
            },
            hash: "".to_string(),
            data,
        };
        block.set_hash();
        block
    }
}