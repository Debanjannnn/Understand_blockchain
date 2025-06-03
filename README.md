# Rust Blockchain Implementation

A simple but functional blockchain implementation in Rust that demonstrates the core concepts of blockchain technology including block creation, mining, and chain validation.

## Features

- ⛓️ Basic blockchain structure with genesis block
- ⚒️ Proof of Work mining algorithm
- 🔐 SHA-256 cryptographic hashing
- 🎯 Adjustable mining difficulty
- 🕒 Timestamp-based block creation
- 📝 JSON serialization support

## Prerequisites

- Rust (nightly version required)
- Cargo (Rust's package manager)

## Dependencies

```toml
[dependencies]
chrono = "0.4"         # For timestamp handling
sha2 = "0.10"          # For SHA-256 hashing
serde = { version = "1.0", features = ["derive"] }  # For serialization
serde_json = "1.0"     # For JSON encoding/decoding
```

## Project Structure

```
src/
├── main.rs              # Entry point
└── models/
    ├── mod.rs          # Module declarations
    ├── block.rs        # Block implementation
    └── blockchain.rs   # Blockchain implementation
```

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd blockchain
```

2. Build the project:
```bash
cargo build
```

## Usage

Run the project:
```bash
cargo +nightly run
```

This will:
1. Create a new blockchain with a genesis block
2. Set the mining difficulty (default: 1)
3. Mine and add a new block to the chain

## Example Output

```
New block added to chain -> Block {
    index: 1,
    timestamp: 1648236489123,
    proof_of_work: 2048,
    previous_hash: "000000a4c2f6284b...",
    hash: "000000f2c6124d8b..."
}
```

## Block Structure

Each block contains:
- `index`: Sequential block number (0 for genesis block)
- `timestamp`: Creation time in milliseconds since Unix epoch
- `proof_of_work`: Number of iterations needed to find a valid hash
- `previous_hash`: Hash of the previous block (empty for genesis)
- `hash`: SHA-256 hash of the block's contents

## Mining Process

1. Create a new block with the next index and previous block's hash
2. Generate an initial hash for the block
3. Increment proof_of_work until the hash starts with the required number of zeros
4. Add the mined block to the chain

## Customization

You can adjust the mining difficulty in `main.rs`:
```rust
let difficulty = 4; // Requires 4 leading zeros in block hashes
```

Higher difficulty values:
- Require more leading zeros in block hashes
- Increase mining time
- Make the blockchain more secure

## Contributing

Contributions are welcome! Here are some ways you can contribute:
- Implement block validation
- Add transaction support
- Implement networking for distributed nodes
- Add consensus mechanisms
- Improve mining efficiency
- Add more tests

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Inspired by the original Bitcoin whitepaper
- Built with Rust's excellent crypto and serialization libraries
