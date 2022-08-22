#!/bin/sh
cd ..
rm -rf ./target ./scripts/*.prof*
export RUSTFLAGS="-Cinstrument-coverage"
export LLVM_PROFILE_FILE="./scripts/rust_practice-%p-%m.profraw"

cargo build
cargo run
grcov . --binary-path ./target/debug -s . -t html --branch --ignore-not-existing -o ./docs/gcov-html/
