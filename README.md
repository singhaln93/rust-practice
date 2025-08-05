[![License](https://img.shields.io/github/license/Neeraj2K18/rust-practice)](https://github.com/Neeraj2K18/rust-practice/blob/main/LICENSE)
[![Issues](https://img.shields.io/github/issues/Neeraj2K18/rust-practice)](https://github.com/Neeraj2K18/rust-practice/issues)
[![Forks](https://img.shields.io/github/forks/Neeraj2K18/rust-practice)](https://github.com/Neeraj2K18/rust-practice/network)
[![Rust Build](https://github.com/Neeraj2K18/rust-practice/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/Neeraj2K18/rust-practice/actions/workflows/rust.yml)
[![Pages Deployment](https://github.com/Neeraj2K18/rust-practice/actions/workflows/pages/pages-build-deployment/badge.svg?branch=main)](https://github.com/Neeraj2K18/rust-practice/actions/workflows/pages/pages-build-deployment)
[![Cargo Docs](https://img.shields.io/badge/cargo--docs-deployed-yellow.svg?branch=main)](https://neeraj2k18.github.io/rust-practice/docs/doxygen-html/doc/rust_practice/index.html)
<!-- [![Code Coverage](https://neeraj2k18.github.io/rust-practice/docs/gcov-html/badges/plastic.svg?branch=main)](https://neeraj2k18.github.io/rust-practice/docs/gcov-html/index.html) -->

📄 View the deployed documentation: 🔗 [singhaln93.github.io/rust-practice](https://singhaln93.github.io/rust-practice/)

---
# rust-practice

## 🦀 Rust Programming Tutorials

Explore the official Rust tutorials and resources to kickstart your journey into systems programming with safety and performance in mind.

### 📘 Official Tutorial
- [Rust Programming Language — rust-lang.org](https://www.rust-lang.org/)  
  The official site offers comprehensive guides, examples, and documentation for all skill levels.

### 📚 Recommended References

1. 🔧 [Rust Starter Pack](https://opheron.github.io/rust-starter-pack/)  
   A curated collection of tools, tips, and learning materials for Rust beginners.

2. 📖 [Rust Language Documentation](http://rust-lang.github.io/rustup/index.html)  
   Learn how to install and manage Rust versions with `rustup`.

3. 📝 [Rust Cheat Sheet](https://cheats.rs/)  
   A handy reference for syntax, commands, and common patterns in Rust.

4. 📦 [The Cargo Book](https://doc.rust-lang.org/cargo/)  
   Master Rust’s package manager and build system with this official guide.

<details>
  <summary><strong>🪟 Windows Installation</strong></summary>

  ### 🛠 Prerequisites
  - Install [Build Tools for Visual Studio 2022](https://visualstudio.microsoft.com/downloads/?q=build+tools)  
    (Make sure to include the “C++ build tools” workload)

  ### 📥 Install Rust
  - Download and run the Rust installer:  
    [`rustup_init.exe`](https://win.rustup.rs/x86_64)

  ### 💻 Recommended VS Code Extensions
  - [Rust (rust-lang.rust)](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)  
  - [rust-analyzer (matklad.rust-analyzer)](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)

</details>

<details>
  <summary><strong>🐧 Linux Installation</strong></summary>

  ### 📥 Install Rust via Terminal
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
</details>


### Rust/Cargo commands
Cargo is rust package manager, which is used to build, run, test, document and install rust applications.
```bash
# Rust toolchain
rustc --version                 # Show Rust compiler version
rustup update                   # Update Rust toolchain to the latest stable version
rustup default <toolchain>      # Set default Rust version (e.g., stable, nightly)
rustup target add <target>      # Add compilation target (e.g., wasm32-unknown-unknown)

# Cargo (package manager)
cargo --version                 # Show Cargo version
cargo new <project_name>        # Create a new project (with VCS by default)
cargo init <project_name>       # Initialize Cargo in an existing directory

# Building and running
cargo build                     # Build debug version
cargo build --release           # Build optimized release version
cargo run                       # Build and run the project
cargo run --release             # Run optimized build
cargo check                     # Check code for errors without producing binaries

# Testing and linting
cargo test                      # Run tests
cargo clippy                    # Run Clippy for static analysis and linting
cargo fmt                       # Format code using rustfmt

# Dependencies
cargo add <crate_name>          # Add a dependency (requires cargo-edit)
cargo update                    # Update dependencies to latest allowed versions
cargo tree                      # View dependency tree (requires cargo-tree)

# Documentation
cargo doc                       # Build documentation for dependencies
cargo doc --open                # Build and open documentation in browser
cargo doc --target-dir=docs     # Output docs to a custom directory
cargo doc --target-dir=docs/doxygen-html

# Publishing and installation
cargo install <crate_name>      # Install a binary crate from crates.io
cargo uninstall <crate_name>    # Uninstall a binary crate
cargo publish                   # Publish a crate to crates.io
cargo login                     # Authenticate for publishing to crates.io

# Install additional components
rustup component add rustfmt rust-analyzer rust-analysis rust-src clippy llvm-tools-preview
# - rustfmt: Formats Rust code (cargo fmt)
# - rust-analyzer: Modern language server for IDEs (better than rls)
# - rust-analysis: Precompiled analysis data to speed up IDEs
# - rust-src: Standard library source code (needed for tools like rust-analyzer)
# - clippy: Linting and static analysis
# - llvm-tools-preview: Required for code coverage and advanced tooling (e.g., grcov)

cargo install grcov             # Install grcov for code coverage

# Cleaning and maintenance
cargo clean                     # Remove target directory (clean build artifacts)
cargo metadata                  # Output project metadata (JSON)

# Misc
cargo bench                     # Run benchmarks
cargo fix                       # Automatically apply lint suggestions
```
For more information on cargo refer [the cargo book](https://doc.rust-lang.org/cargo/) and [crates.io](https://crates.io/)

### Folder structure
    .
    ├── Cargo.lock         # Lockfile: exact versions of dependencies (auto-generated)
    ├── Cargo.toml         # Project manifest: metadata, dependencies, and build configuration
    ├── docs/              # Documentation files (e.g., guides, API docs, design notes)
    ├── LICENSE            # License for the project
    ├── README.md          # Project description and usage instructions
    ├── scripts/           # Utility scripts (e.g., build, deployment, CI)
    ├── src/               # Main source code (default entry: src/main.rs or src/lib.rs)
    ├── target/            # Build artifacts (generated by Cargo)
    └── tests/             # Integration tests (separate from unit tests in `src`)

Use short lowercase names at least for the top-level files and folders except `LICENSE`, `README.md`