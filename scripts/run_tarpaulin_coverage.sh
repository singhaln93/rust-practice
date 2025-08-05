cd ..

echo "Generating tarpaulin report..."
cargo tarpaulin --out html --output-dir ./docs/tarpaulin-html --target-dir ./docs/tarpaulin-html
