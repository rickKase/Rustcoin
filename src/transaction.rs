use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionInput {
    pub previous_output: String,
    pub index: u32,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionOutput {
    pub amount: u64,
    pub recipient: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
    pub timestamp: u64,
}

impl Transaction {
    pub fn hash(&self) -> String {
        let serialized = bincode::serialize(&self).unwrap();
        let hash = Sha256::digest(&serialized);
        format!("{:x}", hash)
    }
}
