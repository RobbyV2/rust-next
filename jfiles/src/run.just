import "../common.just"

# === FULL MODE (default) ===

# Run full-stack development (Rust proxy + Next.js dev)
dev:
  #!/usr/bin/env bash
  set -euo pipefail
  cd "{{ROOT}}"
  echo "Building WASM (dev)..."
  wasm-pack build wasm --target web --out-dir ../public/wasm --dev
  trap 'kill 0' SIGINT
  {{DOTENVX}} bun run dev 2>&1 | sed 's/^/[WEB] /' &
  sleep 2
  {{DOTENVX}} cargo run --bin server 2>&1 | sed 's/^/[RUST] /' &
  wait

# Run full-stack production
prod:
  #!/usr/bin/env bash
  set -euo pipefail
  cd "{{ROOT}}"
  echo "Building WASM..."
  wasm-pack build wasm --target web --out-dir ../public/wasm --release
  echo "Building frontend..."
  {{DOTENVX}} bun run build
  cp -r public .next/standalone/ 2>/dev/null || true
  cp -r .next/static .next/standalone/.next/ 2>/dev/null || true
  echo "Building backend..."
  cargo build --release --bin server
  echo "Starting servers..."
  trap 'kill 0' SIGINT
  {{DOTENVX}} bash -c 'HOSTNAME=${HOST:-0.0.0.0} exec bun ./.next/standalone/server.js' 2>&1 | sed 's/^/[WEB] /' &
  sleep 2
  {{DOTENVX}} ./target/release/server 2>&1 | sed 's/^/[RUST] /' &
  wait

# === API-ONLY MODE ===

# Run API-only development (Rust API server only)
api:
  cd {{ROOT}} && APP_MODE=api-only {{DOTENVX}} cargo run --bin server

# Run API server in release mode
api-release:
  cd {{ROOT}} && APP_MODE=api-only {{DOTENVX}} cargo run --bin server --release

# === FRONTEND-ONLY ===

# Run Next.js dev server only
frontend:
  cd {{ROOT}} && APP_MODE=api-only {{DOTENVX}} bun run dev

# Run Next.js production server
frontend-prod:
  cd {{ROOT}} && {{DOTENVX}} bun run start

# === STATIC MODE (WASM + Next.js, no Rust server) ===

# Run static mode development (WASM + Next.js)
dev-static:
  #!/usr/bin/env bash
  set -euo pipefail
  cd "{{ROOT}}"
  echo "Building WASM (dev)..."
  wasm-pack build wasm --target web --out-dir ../public/wasm --dev
  echo "Starting Next.js dev server..."
  {{DOTENVX}} bun run dev

# Run production (assumes already built)
start-prod:
  #!/usr/bin/env bash
  cd "{{ROOT}}"
  trap 'kill 0' SIGINT
  {{DOTENVX}} bash -c 'HOSTNAME=${HOST:-0.0.0.0} exec bun ./.next/standalone/server.js' 2>&1 | sed 's/^/[WEB] /' &
  sleep 2
  {{DOTENVX}} ./target/release/server 2>&1 | sed 's/^/[RUST] /' &
  wait
