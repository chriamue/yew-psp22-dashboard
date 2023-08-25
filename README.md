# yew-psp22-dashboard
An example and test project showcasing the integration of a PSP22 token smart contract (built with OpenBrush) into a Rust Yew app. This dApp provides a simple dashboard to display the total supply of the token and the balance of the current account.

## Source

This is a changed copy of

https://github.com/paritytech/subxt/tree/master/examples/wasm-example

and

https://github.com/Brushfam/openbrush-contracts/tree/main/examples/psp22_extensions/mintable

## Pre-requisites

- [Rust](https://www.rust-lang.org/tools/install)

```bash
rustup component add rust-src
cargo install --force --locked cargo-contract --version 3.2.0
```

## Test Node

Start a testnode with the following command:

```bash
cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git --tag v0.30.0 --force
substrate-contracts-node --base-path chain
```

## Build the Contract

```bash
cd contract
cargo contract build --release
```

## Test the Contract

```bash
cargo test --release --features e2e-tests
```

### Deploy Contracts

Open the substrate UI at https://contracts-ui.substrate.io/?rpc=ws://127.0.0.1:9944 and deploy the contracts.

## Run DApp

```bash
cd dapp
trunk serve
```

or run the following command to build the dApp and serve it with a local server:

```bash
trunk build --release
cd dist
python3 -m http.server 8080
```

Open the dApp at http://localhost:8080/
