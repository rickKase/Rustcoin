use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use crate::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub previous_hash: String,
    pub timestamp: u64,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(index: u64, previous_hash: String, transactions: Vec<Transaction>, timestamp: u64) -> Block {
        Block {
            index,
            previous_hash,
            timestamp,
            nonce: 0,
            transactions,
        }
    }

    pub fn hash(&self) -> String {
        let serialized = bincode::serialize(&self).unwrap();
        let hash = Sha256::digest(&serialized);
        format!("{:x}", hash)
    }

    pub fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);

        while !self.hash().starts_with(&target) {
            self.nonce += 1;
        }
    }
}
