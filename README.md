# rust-next

Full-stack template: Rust backend (Axum) + Next.js frontend + Rust WASM.

Supports multiple deployment modes via a single codebase.

## Modes

| Mode               | Description                                                      | Use Case                        |
| ------------------ | ---------------------------------------------------------------- | ------------------------------- |
| **full** (default) | Rust server proxies to Next.js. Single entry point on port 3000. | Production apps                 |
| **api-only**       | Rust API standalone. Next.js runs separately with rewrites.      | Microservices, separate deploys |
| **static**         | No Rust server. WASM + Next.js only.                             | GitHub Pages, static hosting    |

Set via `APP_MODE` environment variable.

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Bun](https://bun.sh/) (v1.0+)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [just](https://github.com/casey/just) command runner

### Development

```bash
just src install     # Install all dependencies
just src dev         # Full-stack dev (Rust + Next.js + WASM)
```

Visit `http://localhost:3000`

### Production

```bash
just src build-all   # Build everything
just src prod        # Run production servers
```

## Project Structure

```
.
├── src/
│   ├── api/              # API route handlers (hello, greet, search, create, env)
│   ├── server/           # Server config, routing, and frontend proxy
│   ├── config.rs         # Structured configuration from env vars
│   └── bin/server.rs     # Main entry point
├── wasm/
│   └── src/lib.rs        # Rust WASM exports (greet, add)
├── app/
│   ├── lib/
│   │   ├── api.ts        # Typed API client
│   │   └── basePath.ts   # Runtime base path detection (GitHub Pages)
│   ├── wasm/page.tsx     # WASM demo page
│   ├── page.tsx          # Home page
│   ├── layout.tsx        # Root layout
│   └── globals.css       # Tailwind v4 entry
├── jfiles/               # Justfile modules (build, run, test)
├── Cargo.toml            # Rust workspace
├── package.json          # Frontend dependencies
├── next.config.js        # Next.js config (mode-aware)
└── .github/workflows/    # GitHub Pages deployment
```

## Architecture

### Full Mode (default)

```
Browser → Rust (port 3000) → /api/* handled by Axum
                            → /* proxied to Next.js (port 3001)
```

### API-Only Mode

```
Browser → Next.js (port 3000) → /api/* rewritten to Rust (port 3001)
```

### Static Mode

```
Browser → Next.js / Static Export (WASM loaded from /public/wasm/)
```

## Environment Variables

Copy `.env.example` to `.env.local`:

```env
APP_MODE=full              # full | api-only
SERVER_PORT=3000           # Rust server port
SERVER_HOST=127.0.0.1      # Rust server host (0.0.0.0 for remote)
PORT=3001                  # Next.js server port
HOSTNAME=localhost         # Next.js host (0.0.0.0 for remote)
RUST_LOG=info              # Logging level
```

### GitHub Pages

```env
GITHUB_PAGES=true
NEXT_PUBLIC_BASE_PATH=/your-repo-name
```

## Adding API Routes

1. Create `src/api/my_route.rs`:

```rust
use axum::{Router, response::Json, routing::get};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct MyResponse { message: String }

pub fn routes() -> Router {
    Router::new().route("/my-route", get(handler))
}

async fn handler() -> Json<MyResponse> {
    Json(MyResponse { message: "Hello!".into() })
}
```

2. Register in `src/api/mod.rs`:

```rust
mod my_route;

pub fn routes() -> Router {
    Router::new()
        .merge(hello::routes())
        .merge(my_route::routes())  // add here
}
```

3. Call from frontend via `app/lib/api.ts`:

```typescript
export async function myRoute(): Promise<MyResponse> {
  const response = await fetch('/api/my-route')
  return handleResponse<MyResponse>(response)
}
```

## WASM

Code lives in `wasm/src/lib.rs`. Functions are exported with `#[wasm_bindgen]`.

```bash
just src build-wasm       # Production build
just src build-wasm-dev   # Dev build (faster)
```

The WASM demo page is at `/wasm/`. It uses `basePath` detection for correct asset loading on GitHub Pages.

## Commands

```bash
# Development
just src dev              # Full-stack dev (Rust proxy + Next.js + WASM)
just src dev-static       # Static mode (WASM + Next.js, no Rust server)
just src api              # Rust API server only
just src frontend         # Next.js dev server only

# Production
just src prod             # Build and run full-stack production
just src build-all        # Build everything
just src start-prod       # Run production (pre-built)

# Build
just src build            # Build Rust (release)
just src build-api        # Build API server binary
just src build-frontend   # Build Next.js standalone
just src build-wasm       # Build WASM (release)
just src build-pages      # Build for GitHub Pages (static export)
just src check            # Check Rust without building

# Format & Lint
just src fmt              # Format everything (Rust + TypeScript)
just src fmt-check        # Check formatting
just src fmt-rust         # Rust only
just src fmt-ts           # TypeScript only

# Test
just src test             # Run Rust tests
just src test-wasm        # Run WASM tests

# Maintenance
just src install          # Install all dependencies
just src clean            # Clean build artifacts
```

## Tech Stack

### Backend

- **Axum 0.8** - Web framework with WebSocket support
- **Tokio** - Async runtime
- **Tower-HTTP** - CORS, tracing, static file serving
- **Hyper** - HTTP client for frontend proxying

### WASM

- **wasm-bindgen** - Rust/JS interoperability
- **wasm-pack** - Build tooling

### Frontend

- **Next.js 16** - React framework (App Router)
- **React 19** - UI framework
- **Tailwind CSS v4** - Utility-first CSS
- **TypeScript** - Type safety

## License

MIT
