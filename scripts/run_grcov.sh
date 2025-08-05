#!/bin/sh

# 🚀 Move to project root
cd "$(dirname "$0")/.." || exit 1

# 🧹 Clean previous build artifacts and coverage files
echo "Cleaning previous build and coverage data..."
rm -rf ./target ./scripts/*.prof* ./docs/gcov-html/*

# 🛠️ Set Rust flags for coverage instrumentation
export RUSTFLAGS="-Cinstrument-coverage"
export LLVM_PROFILE_FILE="./scripts/rust_practice-%p-%m.profraw"

# 📦 Build and run the project
echo "Building project..."
cargo build

echo "Running project..."
cargo run

# 📊 Generate coverage report using grcov
echo "Generating coverage report..."
grcov . \
  --binary-path ./target/debug \
  -s . \
  -t html \
  --branch \
  --ignore-not-existing \
  -o ./docs/gcov-html

echo "✅ Coverage report generated at ./docs/gcov-html/index.html"