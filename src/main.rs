mod blockchain;
mod block;
mod transaction;
mod wallet;
mod networking;

use blockchain::Blockchain;
use transaction::{Transaction, TransactionOutput};
use networking::start_server;

#[tokio::main]
async fn main() {
    let mut blockchain = Blockchain::new();

    // Create a sample transaction
    let transaction = Transaction {
        inputs: vec![],
        outputs: vec![TransactionOutput {
            amount: 50,
            recipient: "Alice".to_string(),
        }],
        timestamp: blockchain::now(),
    };

    blockchain.add_block(vec![transaction]);

    println!("Blockchain valid? {}", blockchain.is_valid_chain());

    // Start the P2P server
    start_server().await;
}
