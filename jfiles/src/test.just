import "../common.just"

# Run all Rust tests
test:
  cd {{ROOT}} && cargo test

# Run WASM tests
test-wasm:
  cd {{ROOT}} && wasm-pack test --headless --chrome wasm
