# seed-phrase-generator

🚀 A simple tool to generate cryptographic seed phrases for wallet recovery and authentication. Supports customizable word lists, entropy levels, and BIP-39 standard. 🔐💾


## Features

- Generates cryptographically secure BIP-39 seed phrases
- Supports 12, 15, 18, 21, and 24 word seed phrases
- Uses secure random number generation via `OsRng`
- Simple command-line interface

## Development

```bash
# Clone the repository
git clone https://github.com/patrikduch/seed-phrase-generator.git
cd seed-phrase-generator

# Clean and build the project
cargo clean
cargo run

# Build without running
cargo build
```

### Dependencies

- rand@0.8
- tiny-bip39@1.0

You can easily add dependencies with:

```bash
cargo add rand@0.8 tiny-bip39@1.0
```
