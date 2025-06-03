use super::blockchain::Blockchain;
use chrono::prelude::*;
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};

// Block struct represents a single block in the blockchain
// Example block: {
//   index: 1,
//   timestamp: 1648236489123,
//   proof_of_work: 2048,
//   previous_hash: "000000a4c2f6284b..." (64 chars),
//   hash: "000000f2c6124d8b..." (64 chars)
// }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    // Unique sequential identifier for the block (e.g., 0, 1, 2, ...)
    pub index: u64,
    // Unix timestamp in milliseconds (e.g., 1648236489123)
    pub timestamp: u64,
    // Number of attempts needed to find valid hash (e.g., 2048)
    pub proof_of_work: u64,
    // Hash of the previous block (e.g., "000000a4c2f6284b...")
    pub previous_hash: String,
    // Hash of the current block (e.g., "000000f2c6124d8b...")
    pub hash: String
}

impl Block {
    // Creates a new block with given index and previous block's hash
    // Example: Block::new(1, "000000a4c2f6284b...")
    pub fn new(index: u64, previous_hash: String) -> Self {
        // Initialize block with default values
        let mut block = Block {
            // Sequential block number
            index,
            // Current UTC timestamp in milliseconds
            timestamp: Utc::now().timestamp_millis() as u64,
            // Initial proof of work (will be updated during mining)
            proof_of_work: u64::default(),
            // Hash of the previous block
            previous_hash,
            // Initial empty hash (will be calculated)
            hash: String::default(),
        };
        
        // Calculate initial block hash
        block.hash = block.generate_block_hash();
        block
    }

    // Mines the block by finding a hash with required number of leading zeros
    // Example: block.mine(&blockchain) // blockchain with difficulty 4
    pub fn mine(&mut self, blockchain: &Blockchain) {
        loop {
            // Check if current hash has required number of leading zeros
            // Example: for difficulty 4, hash should start with "0000"
            if !self.hash.starts_with(&"0".repeat(blockchain.difficulty)) {
                // Increment proof of work and recalculate hash
                self.proof_of_work += 1;
                self.hash = self.generate_block_hash();
            } else {
                // Valid hash found, mining complete
                break
            }
        }
    }

    // Generates SHA-256 hash of the block's contents
    // Returns: 64-character hexadecimal hash string
    // Example: "000000f2c6124d8b..."
    pub fn generate_block_hash(&self) -> String {
        // Create a copy of the block for hashing
        let mut block_data = self.clone();
        // Clear the hash field as it shouldn't be part of the new hash
        block_data.hash = String::default();
        // Convert block to JSON string
        // Example: {"index":1,"timestamp":1648236489123,...}
        let serialized_block_data = serde_json::to_string(&block_data).unwrap();

        // Create SHA-256 hasher
        let mut hasher = Sha256::new();
        // Update hasher with block data
        hasher.update(serialized_block_data);
        // Get final hash bytes
        let result = hasher.finalize();
        // Convert hash to hexadecimal string
        format!("{:x}", result)
    }
}