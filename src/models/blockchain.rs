// Import required dependencies
use super::block::Block;  // For Block struct
use chrono::prelude::*;   // For timestamp generation

// Type alias for a vector of blocks
// Example: vec![Block{index: 0, ...}, Block{index: 1, ...}]
type Blocks = Vec<Block>;

// Blockchain struct represents the entire chain of blocks
// Example blockchain: {
//   genesis_block: Block{index: 0, ...},
//   chain: [Block{index: 0, ...}, Block{index: 1, ...}],
//   difficulty: 4
// }
#[derive(Debug, Clone)]
pub struct Blockchain {
    // The first block in the chain (created at initialization)
    // Example: Block{index: 0, hash: "000000abc...", ...}
    pub genesis_block: Block,

    // Vector containing all blocks (including genesis)
    // Example: [Block{index: 0, ...}, Block{index: 1, ...}]
    pub chain: Blocks,

    // Mining difficulty (number of leading zeros required in hash)
    // Example: 4 (requires hashes to start with "0000")
    pub difficulty: usize,
}

impl Blockchain {
    // Creates a new blockchain with specified mining difficulty
    // Example: Blockchain::new(4)
    pub fn new(difficulty: usize) -> Self {
        // Create genesis block with default values
        let mut genesis_block = Block {
            // First block has index 0
            index: 0,
            // Current UTC timestamp in milliseconds
            timestamp: Utc::now().timestamp_millis() as u64,
            // Initial proof of work
            proof_of_work: u64::default(),
            // Genesis block has empty previous hash
            previous_hash: String::default(),
            // Initial empty hash (will be calculated)
            hash: String::default(),
        };
        
        // Calculate genesis block hash
        genesis_block.hash = genesis_block.generate_block_hash();

        // Initialize chain with genesis block
        let mut chain = Vec::new();
        chain.push(genesis_block.clone());

        // Create and return new blockchain instance
        Blockchain {
            genesis_block,
            chain,
            difficulty,
        }
    }

    // Adds a new block to the chain
    // Example: blockchain.add_block()
    pub fn add_block(&mut self) {
        // Get the last block in the chain
        let last_block = self.chain.last().unwrap();
        
        // Create new block with incremented index and previous block's hash
        let mut new_block = Block::new(
            // New block's index is current chain length
            self.chain.len() as u64,
            // Use last block's hash as previous hash
            last_block.hash.clone(),
        );

        // Mine the new block (find valid hash)
        new_block.mine(self);
        // Add the mined block to the chain
        self.chain.push(new_block.clone());
        // Log the newly added block
        println!("New block added to chain -> {:?}", new_block);
    }
}