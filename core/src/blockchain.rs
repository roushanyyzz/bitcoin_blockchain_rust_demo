use crate::block;

pub struct BlockChain {
    pub blocks: Vec<block::Block>,
}

impl BlockChain {
    pub fn add_block(&mut self, data: String) {
        let pre_hash = &self.blocks[&self.blocks.len() - 1].hash;
        let new_block = block::Block::new(data, pre_hash.clone());
        self.blocks.push(new_block);
    }

    fn new_genesis() -> block::Block {
        block::Block::new("This is a genesis block".to_string(), "".to_string())
    }

    pub fn new() -> BlockChain {
        BlockChain {
            blocks: vec![BlockChain::new_genesis()],
        }
    }
}
