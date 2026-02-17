# Solana Substreams Transfers

A Rust-based Substreams module for indexing and analyzing token transfer events on the Solana blockchain. This project enables efficient, real-time streaming of transfer data from Solana transactions using the StreamingFast Substreams framework.

## Overview

This assessment project demonstrates the implementation of a Substreams module that processes Solana blockchain data to extract and index token transfer information. The module leverages Protocol Buffers for data serialization and Rust for high-performance data processing.

## Features

- **Transfer Indexing**: Extracts token transfer events from Solana transactions
- **Real-time Processing**: Streams blockchain data in real-time using Substreams
- **Protocol Buffers**: Type-safe data serialization and deserialization
- **Rust Implementation**: High-performance, memory-safe code execution
- **Solana Integration**: Seamless integration with Solana blockchain data

## Project Structure

```
solana-substreams-transfers/
├── proto/                    # Protocol Buffer definitions for data models
├── src/                      # Rust source code
│   └── lib.rs               # Main module implementation
├── target/                   # Compiled artifacts
├── Cargo.toml               # Rust project manifest
├── Cargo.lock               # Dependency lock file
├── substreams.yaml          # Substreams module configuration
├── buf.gen.yaml             # Protocol Buffer code generation config
└── Makefile                 # Build automation and tasks
```

## Prerequisites

- **Rust**: 1.70+ ([Install Rust](https://www.rust-lang.org/tools/install))
- **Substreams CLI**: Latest version
- **Buf**: For Protocol Buffer compilation
- **Make**: For running build commands

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/aryan735/solana-substreams-transfers.git
   cd solana-substreams-transfers
   ```

2. Install dependencies:
   ```bash
   make install
   ```
   Or manually:
   ```bash
   rustup update
   cargo build
   ```

## Building

Build the Substreams module:

```bash
make build
```

This command:
- Generates Protocol Buffer code from `.proto` files
- Compiles the Rust code
- Creates the Substreams package

## Usage

### Running the Module

Execute the Substreams module against Solana blockchain data:

```bash
substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml map_transfers --start-block 100000000 --stop-block 100010000
```

### Configuration

Edit `substreams.yaml` to modify:
- Module name and version
- Exposed modules and their input/output types
- Module handlers and network settings

## Development

### Proto File Definition

Define your data models in `proto/` files:

```protobuf
syntax = "proto3";

package transfer;

message Transfer {
  string source = 1;
  string destination = 2;
  string mint = 3;
  uint64 amount = 4;
}
```

### Compiling Proto Files

```bash
make proto
```

Or with Buf:
```bash
buf generate
```

### Running Tests

```bash
cargo test
```

## Dependencies

Key dependencies (see `Cargo.toml`):
- **substreams**: Core Substreams framework
- **solana-program**: Solana program utilities
- **prost**: Protocol Buffer implementation
- **tonic**: gRPC framework

## Module Handlers

The module typically exposes handlers for:
- **map_transfers**: Extracts transfer data from transactions
- **store_transfers**: Persists transfer information to a store
- **db_transfers**: Database output for downstream systems

## Performance Considerations

- Optimized Rust compilation for maximum throughput
- Efficient Protocol Buffer serialization
- Streaming architecture for constant memory usage
- Parallelized processing of blockchain blocks

## Troubleshooting

### Build Issues

If you encounter compilation errors:
```bash
cargo clean
cargo build --release
```

### Proto Generation

Ensure Buf is installed and configured:
```bash
buf --version
make proto
```

### Runtime Issues

Check logs for detailed error messages:
```bash
substreams run ... --log-level=debug
```

## Resources

- [Substreams Documentation](https://substreams.streamingfast.io/)
- [Solana Program Library](https://docs.rs/solana-program/)
- [Protocol Buffers Guide](https://developers.google.com/protocol-buffers)
- [Rust Book](https://doc.rust-lang.org/book/)

## License

This project is provided as-is for educational and assessment purposes.

## Support

For issues, questions, or contributions, please refer to the repository issues section.

---

**Last Updated**: February 2026
