# rust-next

Next.js frontend with Rust backend API server using file-system based routing.

## Branches

- **main**: The active development branch (Rust Backend + Next.js Frontend).
- **static**: Rust WASM + Next.js (no backend server). Ideal for static hosting.
- **single-server**: (If applicable) A variation with a single server setup.

## Architecture

- **Frontend**: Next.js (React) on port 3000
- **Backend**: Rust API server (Axum) on port 3001
- **Routing**: File-system based - each file in `src/api/` defines routes for an endpoint
- API requests from Next.js are automatically proxied to the Rust backend

## Project Structure

```
rust-next/
├── app/                      # Next.js app directory
│   ├── page.tsx             # Main page
│   └── layout.tsx           # Root layout
├── src/
│   ├── lib.rs               # Library entry point
│   ├── api/                 # API route handlers (file-system based routing)
│   │   ├── mod.rs          # Module declarations
│   │   ├── hello.rs        # GET /api/hello
│   │   ├── greet.rs        # GET /api/greet/:name
│   │   ├── search.rs       # GET /api/search?q=...
│   │   └── create.rs       # POST /api/create
│   ├── server/              # Server infrastructure
│   │   ├── mod.rs
│   │   └── route_builder.rs # Route registration
│   └── bin/
│       └── server.rs        # Main server binary
├── Cargo.toml               # Rust dependencies
├── package.json             # Node.js dependencies
└── next.config.js           # Next.js config with API proxy
```

## Key Features

✅ **File-system based routing** - Each file in `src/api/` corresponds to API routes  
✅ **Automatic binary discovery** - No need to manually specify binaries in `Cargo.toml`  
✅ **Clean separation** - Next.js handles UI, Rust handles API logic  
✅ **Flexible route definitions** - Define one or multiple routes per file  
✅ **Production-ready builds** - Optimized release builds included

## Prerequisites

- [Rust](https://rustup.rs/) (rustc 1.70+)
- [Bun](https://bun.sh/) (v1.0+)
- Optional: [just](https://github.com/casey/just) command runner

## Getting Started

### 1. Setup Environment Variables

```bash
# Copy the example environment file
cp .env.example .env.local

# Edit .env.local with your values (optional - defaults work for local dev)
```

**Note:** `.env.example` is the template (committed to git), `.env.local` is your actual values (gitignored).

### 2. Install Dependencies

```bash
# Install dependencies with Bun
bun install

# Rust dependencies will be installed on first build
```

### 3. Development Mode

**Option A: Run both servers together (Recommended)**

```bash
just src dev
```

This runs both the Rust API and Next.js frontend in one terminal with prefixed logs:

- `[API]` - Rust server logs
- `[WEB]` - Next.js logs

Press `Ctrl+C` to stop both servers at once.

> **Note:** `just src dev` requires Bash (e.g. Git Bash), WSL, Linux, or macOS. It will NOT work in PowerShell or CMD.
> For PowerShell/CMD, use Option B (run servers separately).

**Option B: Run servers separately**

**Terminal 1 - Rust API Server:**

```bash
cargo run --bin server
# OR with just:
just src api
```

**Terminal 2 - Next.js Frontend:**

```bash
bun run dev
# OR with just:
just src frontend
```

Then open [http://localhost:3000](http://localhost:3000) in your browser.

### 4. Production Build

```bash
# Build everything
just src build-all

# OR manually:
cargo build --release --bin server
bun run build

# Run production servers
./target/release/server    # Terminal 1
bun start                   # Terminal 2
```

## API Endpoints

The Rust API server provides these example endpoints:

- `GET /api/hello` - Simple hello message
- `GET /api/greet/:name` - Greet with a name parameter
- `GET /api/search?q=<query>` - Search with query parameter
- `POST /api/create` - Create resource (expects JSON: `{"name": "value"}`)

## Adding New API Routes

### Simple Approach (One Route Per File)

1. Create a new file in `src/api/`:

```bash
touch src/api/users.rs
```

2. Define the route handler:

```rust
// src/api/users.rs
use axum::{response::Json, routing::get, Router};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

pub fn routes() -> Router {
    Router::new().route("/api/users", get(handler))
}

async fn handler() -> Json<Vec<User>> {
    Json(vec![
        User { id: 1, name: "Alice".to_string() },
        User { id: 2, name: "Bob".to_string() },
    ])
}
```

3. Add to module declarations in `src/api/mod.rs`:

```rust
pub mod users;
```

4. Register in `src/server/route_builder.rs`:

```rust
pub fn register_routes() -> Router {
    Router::new()
        // ... existing routes
        .merge(crate::api::users::routes())
}
```

### Advanced Approach (Multiple Routes Per File)

You can define multiple related routes in a single file:

```rust
// src/api/products.rs
use axum::{
    extract::Path,
    response::Json,
    routing::{get, post, delete},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/api/products", get(list_products).post(create_product))
        .route("/api/products/:id", get(get_product).delete(delete_product))
}

async fn list_products() -> Json<Vec<String>> {
    Json(vec!["Product 1".to_string()])
}

async fn create_product() -> Json<String> {
    Json("Created".to_string())
}

async fn get_product(Path(id): Path<u32>) -> Json<String> {
    Json(format!("Product {}", id))
}

async fn delete_product(Path(id): Path<u32>) -> Json<String> {
    Json(format!("Deleted {}", id))
}
```

## Development Commands

### Using Just

```bash
just                        # List all available commands
just src                    # List all src commands

# Development (run.just)
just src dev                # Run BOTH servers together (Bash/WSL/Unix only!)
just src api                # Run Rust API only
just src frontend           # Run Next.js only
just src api-release        # Run Rust API (release mode)
just src frontend-prod      # Run Next.js (production mode)

# Build (build.just)
just src build              # Build Rust for production
just src build-api          # Build Rust API for production
just src build-frontend     # Build Next.js for production
just src build-all          # Build both for production
just src check              # Check Rust code without building

# Format & Lint (build.just)
just src fmt                # Format and lint ALL code (Rust + TypeScript)
just src fmt-check          # Check formatting without changes
just src fmt-rust           # Format Rust only
just src fmt-ts             # Format TypeScript only

# Test (test.just)
just src test               # Run Rust tests

# Maintenance (justfile)
just src install            # Install dependencies
just src clean              # Clean build artifacts
```

### Using Cargo/Bun Directly

**Rust:**

```bash
cargo run --bin server              # Development
cargo run --bin server --release    # Production mode
cargo build --release --bin server  # Production build
cargo check                          # Check for errors
cargo test                           # Run tests
cargo fmt                            # Format code
cargo clippy                         # Lint code
```

**Next.js:**

```bash
bun run dev          # Development server
bun run build        # Production build
bun start            # Production server
bun run lint         # Lint code
bun run format       # Format code with Prettier
```

## Code Quality

### Pre-commit Hooks

This project uses [pre-commit](https://pre-commit.com/) for git hooks that automatically check:

- ✅ Rust formatting (`cargo fmt`)
- ✅ Rust linting (`cargo clippy`)
- ✅ TOML formatting (`taplo fmt`)
- ✅ TypeScript linting (`eslint`)
- ✅ Code formatting (`prettier`)

**Setup:**

```bash
# Install pre-commit (if not already installed)
pip install pre-commit
# OR
brew install pre-commit

# Install the git hooks
pre-commit install
```

**Manual formatting:**

```bash
just src fmt         # Format all code (Rust + TypeScript)
just src fmt-rust    # Format Rust only
just src fmt-ts      # Format TypeScript only
```

**Run hooks manually:**

```bash
pre-commit run --all-files   # Run all hooks on all files
just src fmt-check           # Or use just command
```

## Tech Stack

### Backend (Rust)

- **axum**: Modern web framework
- **tokio**: Async runtime
- **serde/serde_json**: Serialization
- **tower-http**: HTTP middleware (CORS, tracing)
- **tracing**: Structured logging

### Frontend (Next.js)

- **React 19**: UI framework
- **Next.js 16**: React framework with App Router
- **TypeScript**: Type safety

## Directory-Based Routing Philosophy

The routing structure mirrors the file system in `src/api/`:

- Each file exports a `routes()` function that returns an `axum::Router`
- Routes can be simple (one handler) or complex (multiple related handlers)
- Import/export utilities and share code between route files as needed
- The `route_builder.rs` acts as a central registry

This approach gives you:

- Clear organization (one file per feature/endpoint)
- Flexibility (define one or many routes per file)
- Explicit control (routes are manually registered in route_builder)
- Easy refactoring (move route files around as your API grows)

## Production Deployment

### Rust API Binary

```bash
cargo build --release --bin server
# Binary location: ./target/release/server
# Run: ./target/release/server
```

### Next.js

```bash
bun run build
bun start
# OR deploy to Vercel, Netlify, etc.
```

The production Rust binary is optimized and typically 5-10MB in size.
