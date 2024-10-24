# koru-core

Core smart contracts for the Koru Protocol.

## Project Structure

This repository uses the recommended structure for a Soroban project:

```text
.
├── contracts
│   └── user-contract
│       ├── src
│       │   ├── lib.rs
│       │   ├── bridge
│       │   │   └── ..
│       │   ├── methods
│       │   │   └── ..
│       │   ├── storage
│       │   │   ├── ..
│       │   │   └── types
│       │   │       └── ..
│       │   └── tests
│       │       ├── config
│       │       │   └── contract.rs
│       │       └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `user-contract` contract in there to get you started.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.

## Contract Deployment

To deploy this contract it is needed to call the **stellar CLI**:

## Previous steps

Before proceeding, ensure that you have installed the **Stellar CLI**. This is essential for the following operations.

[Set up your smart contract environment](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup)

Make sure to install the required target as specified in the setup documentation. This is crucial for compiling and deploying your smart contracts.

[Target instalation setup](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup#install-the-target)

### Run the following command:

After completing the setup steps, you can verify that the **Stellar CLI** is installed correctly by checking the version.

```bash
stellar --version
```

#### Expected Output

**The output should look similar to this (note that your versions may vary):**

```textplain
stellar 21.2.0 (622c9915ac8c9645e53fe64cf5df73283630ff15)
soroban-env 21.2.0 (8809852dcf8489f99407a5ceac12625ee3d14693)
soroban-env interface version 90194313216
stellar-xdr 21.2.0 (9bea881f2057e412fdbb98875841626bf77b4b88)
xdr curr (70180d5e8d9caee9e8645ed8a38c36a8cf403cd9)
```

If your output matches this format, your CLI installation is successfully verified, and you're ready to proceed.

### Tesnet config

```bash
stellar network add <NAME> --global --rpc-url <URL> --network-passphrase <PASSPHRASE>
```

### Arguments

- `<NAME>`: The name of the network to be added. E.g.: **pubnet**
- `--rpc-url <URL>`: The RPC URL of the network.
- `--network-passphrase <PASSPHRASE>`: The passphrase of the network.

### Example

```bash
stellar network add testnet --global --rpc-url https://soroban-testnet.stellar.org --network-passphrase "Test SDF Network ; September 2015"
```

#### Output

```text
Global "<toml-file-path>.toml"
Name: testnet
Network {
    rpc_url: "https://soroban-testnet.stellar.org:443",
    network_passphrase: "Test SDF Network ; September 2015",
}
```

[Available passphrases](https://developers.stellar.org/docs/learn/encyclopedia/network-configuration/network-passphrases)

#### The network is added globally.

### Deploy a Smart contract

#### Example

```bash
stellar contract deploy --wasm target/wasm32-unknown-unknown-release/{ contract file name }.wasm --network testnet --source S...
```

#### Output

```text
CDJRRYST3MS2XFLZBT373YZ77A2HCJKKM4JAFUR2F27AUWID6NKJYB2V
```

## Install the .wasm file into the network

To install the wasm file in the network for deployments with deployer contract:

```bash
stellar contract install --wasm <WASM-PATH> --network <NETWORK> --source <SOURCE-ACCOUNT-SECRET>
```

#### Example

```bash
stellar contract install --wasm target/wasm32-unknown-unknown-release/{ contract file name }.wasm --network testnet --source S...
```

#### Output

```text
db29620a807d9bb79f8da652970c20bede68c896f8524ac0b5bc4fb724b289b0
```
