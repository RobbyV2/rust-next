# rust-next

Full-stack template: Rust backend (Axum) + Next.js frontend + Rust WASM.

Supports multiple deployment modes via a single codebase. Includes OpenAPI/Swagger UI, per-IP rate limiting, and Docker support out of the box.

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

### Docker

```bash
just src docker      # Build Docker image
just src docker-run  # Run with docker compose
```

Or directly:

```bash
docker build -t rust-next .
docker run -p 3000:3000 rust-next
```

The Docker image uses a multi-stage build with [cargo-chef](https://github.com/LukeMathWalker/cargo-chef) for dependency caching. The runtime image is ~380MB based on `debian:bookworm-slim`. Set `APP_MODE=api-only` to run only the Rust server (no Next.js).

## Project Structure

```
.
├── src/
│   ├── api/              # API route handlers (hello, greet, search, create, env)
│   │   └── openapi.rs    # OpenAPI spec + Swagger UI
│   ├── server/           # Routing, rate limiting, and frontend proxy
│   ├── config.rs         # Hierarchical config (defaults → env → CLI)
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
├── Dockerfile            # Multi-stage production build
├── docker-compose.yml    # Container orchestration
└── .github/workflows/    # GitHub Pages deployment
```

## Architecture

### Full Mode (default)

```
Browser → Rust (port 3000) → /api/* handled by Axum (rate limited)
                            → /api/swagger-ui/ → Swagger UI
                            → /api/openapi.json → OpenAPI spec
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

## API Documentation

Swagger UI is available at `/api/swagger-ui/` when the server is running. The OpenAPI spec is served at `/api/openapi.json`.

Swagger UI is enabled by default. Disable with:

```env
SWAGGER_UI=false
```

All API handlers are annotated with [utoipa](https://github.com/juhaku/utoipa) macros, so the spec stays in sync with the code automatically.

## Rate Limiting

API routes are rate limited per IP using [tower-governor](https://github.com/benwis/tower-governor). Defaults: 10-request burst, replenishing at 2 requests/second.

Configure via environment variables:

```env
RATE_LIMIT_PER_SECOND=2    # Token replenish rate
RATE_LIMIT_BURST=10         # Max burst before 429 Too Many Requests
```

Rate limiting applies only to `/api/*` routes, not the frontend proxy.

## Environment Variables

Copy `.env.example` to `.env.local`:

```env
APP_MODE=full              # full | api-only
HOST=127.0.0.1             # Bind address for both servers (0.0.0.0 for remote)
SERVER_PORT=3000           # Rust server port
PORT=3001                  # Next.js server port
RATE_LIMIT_PER_SECOND=2    # Rate limit replenish rate
RATE_LIMIT_BURST=10        # Rate limit burst size
SWAGGER_UI=false           # Disable Swagger UI (enabled by default)
RUST_LOG=info              # Logging level
```

### CLI Overrides

```bash
./target/release/server --port 8080 --host 0.0.0.0 --mode api-only
```

### GitHub Pages

```env
GITHUB_PAGES=true
NEXT_PUBLIC_BASE_PATH=/your-repo-name
```

## Adding API Routes

1. Create `src/api/my_route.rs`:

```rust
use axum::response::Json;
use super::ApiResponse;

#[utoipa::path(get, path = "/api/my-route", responses((status = 200, body = ApiResponse)))]
pub(crate) async fn handler() -> Json<ApiResponse> {
    Json(ApiResponse { message: "Hello!".into(), data: None })
}
```

2. Register in `src/api/mod.rs`:

```rust
pub(crate) mod my_route;

pub fn routes() -> Router {
    Router::new()
        .route("/hello", get(hello::handler))
        .route("/my-route", get(my_route::handler))  // add here
        // ...
}
```

3. Add to OpenAPI spec in `src/api/openapi.rs`:

```rust
#[openapi(
    paths(
        super::hello::handler,
        super::my_route::handler,  // add here
        // ...
    ),
    // ...
)]
```

4. Call from frontend via `app/lib/api.ts`:

```typescript
export async function myRoute(): Promise<ApiResponse> {
  const response = await fetch('/api/my-route')
  return handleResponse<ApiResponse>(response)
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

# Docker
just src docker           # Build Docker image
just src docker-run       # Run with docker compose
just src docker-up        # Run detached
just src docker-down      # Stop container

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
- **utoipa 5** - OpenAPI spec generation + Swagger UI
- **tower-governor** - Per-IP rate limiting (governor-based)
- **config + clap** - Hierarchical configuration (env → CLI overrides)

### WASM

- **wasm-bindgen** - Rust/JS interoperability
- **wasm-pack** - Build tooling

### Frontend

- **Next.js 16** - React framework (App Router)
- **React 19** - UI framework
- **Tailwind CSS v4** - Utility-first CSS
- **TypeScript** - Type safety

### Infrastructure

- **Docker** - Multi-stage build with cargo-chef dependency caching
- **GitHub Actions** - GitHub Pages deployment workflow
