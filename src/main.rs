// Import the models module containing blockchain implementation
mod models;

// Import Blockchain struct from the models module
use crate::models::blockchain::Blockchain;

fn main() {
    // Set mining difficulty (number of leading zeros required in block hashes)
    // Example: 4 will require hashes like "0000abc..."
    let difficulty = 1;

    // Create a new blockchain with specified difficulty
    // Example: Blockchain{genesis_block: Block{index: 0, ...}, chain: [...], difficulty: 1}
    let mut blockchain = Blockchain::new(difficulty);

    // Add a new block to the chain
    // This will:
    // 1. Create a new block with index 1
    // 2. Mine it until a valid hash is found
    // 3. Add it to the chain
    blockchain.add_block();
}