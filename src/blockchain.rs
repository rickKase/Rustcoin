use crate::block::Block;
use crate::transaction::Transaction;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let genesis_block = Block::new(0, String::from("0"), vec![], now());
        Blockchain {
            chain: vec![genesis_block],
            difficulty: 4,
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let last_block = self.chain.last().unwrap();
        let mut new_block = Block::new(
            last_block.index + 1,
            last_block.hash(),
            transactions,
            now(),
        );

        new_block.mine(self.difficulty);
        self.chain.push(new_block);
    }

    pub fn is_valid_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.previous_hash != previous_block.hash() {
                return false;
            }
        }
        true
    }
}

pub fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}
