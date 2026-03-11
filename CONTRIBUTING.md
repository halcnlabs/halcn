# Contributing to halcn

Contributions are welcome. Please follow these guidelines:

## Setup

1. Clone the repository
2. Install Rust (stable) and Solana CLI
3. Install Node.js 20+ for SDK development
4. Run `scripts/setup-validator.sh` for local testing

## Code Style

- Rust: `cargo fmt` and `cargo clippy` must pass with no warnings
- TypeScript: follow the existing style in `sdk/src/`

## Pull Requests

- One feature or fix per PR
- Include tests for new functionality
- Update documentation if public APIs change
- All CI checks must pass before merge

## Testing

Run the full test suite:

```bash
./scripts/test.sh
```

Or run Rust and SDK tests separately:

```bash
cargo test --workspace
cd sdk && npm test
```
