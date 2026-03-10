import "../common.just"

# Build Rust server for production
build:
  cd {{ROOT}} && cargo build --release

# Build WASM module (release)
build-wasm:
  cd {{ROOT}} && wasm-pack build wasm --target web --out-dir ../public/wasm --release

# Build WASM module (dev, faster)
build-wasm-dev:
  cd {{ROOT}} && wasm-pack build wasm --target web --out-dir ../public/wasm --dev

# Build the API server binary
build-api:
  cd {{ROOT}} && cargo build --release --bin server

# Build Next.js for production (standalone)
build-frontend:
  cd {{ROOT}} && bun run build
  @echo "Copying static assets for standalone build..."
  cd {{ROOT}} && cp -r .next/static .next/standalone/.next/static
  cd {{ROOT}} && cp -r public .next/standalone/public

# Build Next.js for GitHub Pages (static export)
build-pages:
  cd {{ROOT}} && GITHUB_PAGES=true bun run build

# Build everything for production (full mode)
build-all: build-wasm build-api build-frontend
  @echo ""
  @echo "Production build complete!"
  @echo "  API binary:  {{ROOT}}/target/release/server"
  @echo "  WASM:        {{ROOT}}/public/wasm/"
  @echo "  Frontend:    {{ROOT}}/.next/standalone/"
  @echo ""
  @echo "Run with: just src prod"

# Check Rust code without building
check:
  cd {{ROOT}} && cargo check

# Format and lint all code
fmt:
  cd {{ROOT}} && cargo clippy --all-targets --all-features --fix --allow-dirty -- -D warnings
  cd {{ROOT}} && cargo fmt --all
  cd {{ROOT}} && taplo fmt
  cd {{ROOT}} && bunx prettier --write "**/*.{ts,tsx,js,jsx,json,md,mjs}"

# Check formatting and linting without changes
fmt-check:
  cd {{ROOT}} && cargo fmt --check --all
  cd {{ROOT}} && cargo clippy --all-targets --all-features -- -D warnings
  cd {{ROOT}} && taplo fmt --check
  cd {{ROOT}} && bunx prettier --check "**/*.{ts,tsx,js,jsx,json,md,mjs}"

# Format Rust only
fmt-rust:
  cd {{ROOT}} && cargo clippy --all-targets --all-features --fix --allow-dirty -- -D warnings
  cd {{ROOT}} && cargo fmt --all
  cd {{ROOT}} && taplo fmt

# Format TypeScript/frontend only
fmt-ts:
  cd {{ROOT}} && bun run lint --fix
  cd {{ROOT}} && bunx prettier --write "**/*.{ts,tsx,js,jsx,json,md,mjs}"

# Build Docker image
docker:
  cd {{ROOT}} && docker build -t rust-next .

# Run Docker container
docker-run:
  cd {{ROOT}} && docker compose up

# Run Docker container (detached)
docker-up:
  cd {{ROOT}} && docker compose up -d

# Stop Docker container
docker-down:
  cd {{ROOT}} && docker compose down

# Clean build artifacts
clean:
  cd {{ROOT}} && cargo clean
  cd {{ROOT}} && rm -rf .next out node_modules/.cache public/wasm
