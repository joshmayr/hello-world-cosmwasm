# Hello, World! CosmWasm Smart Contract
---

This project demonstrates a basic smart contract written in CosmWasm. It comes with a smart contract, helper functions to interact with the deployed contract, and a test for the contract.

---

## Setup the Project

Before you can run the project test cases you will need to install dependencies and setup your keypair.

### Install Rust
First make sure you have the Rust programming language installed, you can follow the installation guide [here](https://doc.rust-lang.org/book/ch01-01-installation.html?utm_source=buildspace.so&utm_medium=buildspace_project): 

Verify that Rust and Cargo were installed correctly by running the following commands.
```shell
rustup --version
rustc --version
cargo --version
```

Install cargo-generate and cargo-run-script
```shell
cargo install cargo-generate --features vendored-openssl
cargo install cargo-run-script
```

---

### Common Commands

Generate schema
```shell
cargo schema
```

Build the contract
```shell
cargo wasm
```

Run test cases
```shell
RUST_BACKTRACE=1 cargo unit-test
```