# Rust WASM + Next.js Template

Rust WASM + Next.js template for client-side Rust applications.

## Branches

- **main**: The active development branch.
- **static**: Rust WASM + Next.js (no backend server). Ideal for static hosting or client-side logic.
- **single-server**: (If applicable) A variation with a single server setup.

## Tech Stack

Rust WASM + Next.js

## Running

```bash
# Build WASM files first
wasm-pack build --target web --out-dir public/wasm --release

# Development
just src dev

# Production
just src prod
```

Visit http://localhost:3000

## Project Structure

```
.
├── app/                 # Next.js app directory
│   ├── page.tsx        # Main page
│   └── layout.tsx      # Root layout
├── wasm/               # Rust WASM source
│   └── src/lib.rs     # WASM entry point
├── public/wasm/        # Built WASM output (gitignored)
├── Cargo.toml          # Rust dependencies and config
├── package.json        # Node.js dependencies
└── justfile            # Build and run commands
```

## Development Commands

```bash
just src dev          # Build WASM + run dev server
just src build-wasm   # Build WASM only (release)
just src build-wasm-dev # Build WASM only (dev)
just src frontend     # Run Next.js dev server only
just src prod         # Build and run production
just src build        # Build everything for production
just src fmt          # Format all code
just src test         # Run tests
```

## Prerequisites

- [Rust](https://rustup.rs/) (rustc 1.70+)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Bun](https://bun.sh/) (v1.0+)
- [just](https://github.com/casey/just) command runner

## Adding WASM Features

1. Add Rust code to `wasm/src/lib.rs`
2. Export functions with `#[wasm_bindgen]`
3. Rebuild WASM: `just src build-wasm`
4. Import in Next.js and use

Example:

```rust
// wasm/src/lib.rs
#[wasm_bindgen]
pub fn my_function(input: &str) -> String {
    format!("Processed: {}", input)
}
```

```typescript
// app/page.tsx
const wasm = await import('/wasm/rust_next_wasm.js')
await wasm.default('/wasm/rust_next_wasm_bg.wasm')
const result = wasm.my_function('hello')
```

## License

MIT
