# Abstract Extension Module Template
This is a repository for developing an extension using [Abstract-OS](https://abstract.money).
An extension is a non-user-migratable module that (typically) provides an adapter between external services and the OS.
This repository includes a template for the extension module, deployment interfaces, and deployment scripts.

## Features
- Abstracted handlers for `instantiate`, `query`, `execute`, `migrate`, and `replies`
- Deployment scripts
- Asset value calculation using Abstract Name Service (ANS)


## Instructions

### Option 1
Use the template using [`cargo generate`](https://cargo-generate.github.io/cargo-generate/index.html)
```shell
cargo generate gh:Abstract-OS/extension-template -b template
```
### Option 2:
Fork this repository


## Commands
- `cargo build`: Build the app
- `cargo test`: Run the tests
- `cargo deploy`: Deploy the extension to Abstract Version Control
- `cargo wasm`: Compile the app to wasm
- `cargo doc`: Generate the documentation
- `cargo clippy`: Run the linter

## References
- Coming soon
