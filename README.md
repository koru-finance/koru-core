# koru-core

Core smart contracts for the Koru Protocol.

## Project Structure

This repository uses the recommended structure for a Soroban project:

```text
.
├── contracts
│   └── user-contract
│       ├── src
│       │   ├── lib.rs
│       │   └── test
│       │       ├── config
│       │       │   └── contract.rs
│       │       └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `user-contract` contract in there to get you started.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
