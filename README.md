# Paperback Toolchain RS

`paperback-toolchain-rs` is a Rust-based toolchain designed to streamline your development process. This project aims to provide Rust developers with a set of tools and libraries that enhance productivity and code quality.

## Installation

To install `paperback-toolchain-rs`, you need to have Rust and Cargo installed on your machine. If you haven't already, you can install Rust and Cargo by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

Once Rust and Cargo are installed, you can install `paperback-toolchain-rs` by cloning this repository and running the following command in the project directory:

```bash
cargo build --release
```

## Usage
After installing, you can run paperback-toolchain-rs using Cargo:
```bash
cargo run
```
This command will execute the main functionality of the toolchain.

## Roadmap
- [ ] Create basic Command Line Interface
- Implement core functionality:
  - [ ] Migrating paperback v0.8 extensions to paperback v0.9 extensions
  - [ ] Publish CLI to NPM (npx ...)
    - [ ] Add napi-rs bindings for CLI to NPM
  - [ ] Creating a new project with a template
    - [ ] Core optional logic components i.e. interceptors
  - [ ] Building a project
  - [ ] Running / serving a project
  - [ ] Testing a project
- [ ] Create a basic GUI using Flutter

## Disclaimer
This project makes use of rolldown, which is not production-ready.
Therefore, some bugs may arise from the use of rolldown.
 
## License
[MIT](LICENSE.md)
