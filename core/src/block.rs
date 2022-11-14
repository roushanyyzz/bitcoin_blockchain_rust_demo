use chrono::Utc;
// use serde::Serialize;
use utils::coder::{my_serialize, Serialize};
use utils::hash::get_hash;

#[derive(Serialize, Debug)]
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
    pub fn new(data: String, pre_hash: String) -> Block {
        let data_serialize = my_serialize(&data);
        let tx_hash = get_hash(&data_serialize);
        let header = BlockHeader {
            time: Utc::now().timestamp(),
            tx_hash,
            pre_hash,
        };
        let header_serialzie = my_serialize(&header);
        let header_hash = get_hash(&header_serialzie);
        Block {
            header,
            data,
            hash: header_hash,
        }
    }
}

#[cfg(test)]
mod block_test {

    use super::Block;

    #[test]
    fn block_work() {
        let block = Block::new(
            "data".to_string(),
            String::from("63c222837c6c8ce660c2d9b974e3d6d2935e3697e03a19493319cc5e03ff017e"),
        );
        println!("{:?}", block);
    }
}
