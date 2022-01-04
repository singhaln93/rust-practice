# rust-practice

Rust Programming Tutorials [rust-lang official website](https://www.rust-lang.org/)

References:
  1. [Rust Lang documentation](http://rust-lang.github.io/rustup/index.html)

<details>
  <summary>Windows</summary>

  ### Installation
Install `rustup`
</details>

<details>
  <summary>Linux</summary>

### Installation
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
</details>

### Rust/Cargo commands
```bash
rustc --version             # Get rust compiler version
rustup update               # Update rust compiler
cargo --version             # Get cargo package manager version
cargo init <project_name>   # Init sample project
cargo build --release       # Build release (optimized)
cargo run                   # Execute rust application or program
```

### Rust Code Formatting
You can use `format document` of vscode or `rustfmt`
```bash
rustup component add rustfmt #add rustfmt component
cargo fmt                    #cargo format the project
```