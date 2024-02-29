use chrono::prelude::*;
use utils::coder;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BlockHeader {
    pub time: i64,
    pub tx_hash: String,
    pub pre_has: String,
}

#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub data: String,
}

impl Block{
    fn set_hash(&mut self){
        self.header.time = Utc::now().timestamp();
        let header = coder::my_serialize(&self.header);
        self.hash = coder::get_hash(&header[..]);
    }

    pub fn new_block(data: String, pre_hash: String) -> Block {
        let transaction = coder::my_serialize(&data);
        let tx_hash = coder::get_hash(&transaction[..]);
        let time = Utc::now().timestamp();

        let mut block = Block {
            header: BlockHeader {
                time: time,
                tx_hash: tx_hash,
                pre_has: pre_hash,
            },
            hash: "".to_string(),
            data: data,
        };
        block.set_hash();
        block
    }

}