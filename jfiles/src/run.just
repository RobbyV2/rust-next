import "../common.just"

# Run commands

# Run both API and frontend together (Bash/WSL/Linux/macOS only)
dev:
  #!/usr/bin/env bash
  set -euo pipefail
  cd "{{ROOT}}"
  trap 'kill 0' SIGINT
  cargo run --bin server 2>&1 | sed 's/^/[API] /' &
  sleep 2
  bun run dev 2>&1 | sed 's/^/[WEB] /' &
  wait

# Run Rust API server (development)
api:
  cd {{ROOT}} && cargo run --bin server

# Run Rust API server (release mode)
api-release:
  cd {{ROOT}} && cargo run --bin server --release

# Run Next.js dev server
frontend:
  cd {{ROOT}} && bun run dev

# Run Next.js production server
frontend-prod:
  cd {{ROOT}} && bun start

