import "../common.just"

# Run commands

# Run development mode (build WASM dev + Next.js dev server)
dev:
  #!/usr/bin/env bash
  set -euo pipefail
  cd "{{ROOT}}"
  echo "Building WASM (dev)..."
  wasm-pack build --target web --out-dir public/wasm --dev
  echo "Starting Next.js dev server..."
  bun run dev

# Run Next.js dev server only (assumes WASM already built)
frontend:
  cd {{ROOT}} && bun run dev

# Build and run production
prod:
  #!/usr/bin/env bash
  set -euo pipefail
  cd "{{ROOT}}"
  echo "Building WASM..."
  wasm-pack build --target web --out-dir public/wasm --release
  echo "Building frontend..."
  bun run build
  echo "Starting server..."
  bun run start

# Run production (assumes already built)
start-prod:
  cd {{ROOT}} && bun run start
