use crate::block::Block;

use std::collections::HashMap;

pub struct BlockManager {
    blocks: HashMap<u32, Block>
}

impl BlockManager { 
    pub fn new() -> BlockManager {
        BlockManager { blocks: HashMap::new() }
    }

    pub fn add_block(&mut self, id: u32, block: Block) {
        self.blocks.insert(id, block);
    }

    pub fn get_block(&self, id: &u32) -> Option<&Block> {
        self.blocks.get(id)
    }
}
