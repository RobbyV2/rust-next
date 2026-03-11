# Stage 1: Prepare dependency recipe
FROM rust:slim-bookworm AS chef
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-chef
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
WORKDIR /app

# Stage 2: Generate recipe from workspace
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3: Build dependencies + server + WASM
FROM chef AS rust-builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin server
RUN wasm-pack build wasm --target web --out-dir ../public/wasm --release

# Stage 4: Build Next.js
FROM oven/bun:latest AS js-builder
WORKDIR /app

COPY package.json bun.lock ./
RUN bun install --frozen-lockfile

COPY . ./
COPY --from=rust-builder /app/public/wasm ./public/wasm

RUN bun run build

# Stage 5: Runtime
FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y ca-certificates curl && rm -rf /var/lib/apt/lists/*

COPY --from=oven/bun:latest /usr/local/bin/bun /usr/local/bin/bun

ENV NODE_ENV=production
ENV NEXT_TELEMETRY_DISABLED=1
ENV HOST=0.0.0.0
ENV SERVER_PORT=3000
ENV PORT=3001
ENV APP_MODE=full

COPY --from=js-builder /app/.next/standalone ./
COPY --from=js-builder /app/.next/static ./.next/static
COPY --from=js-builder /app/public ./public
COPY --from=rust-builder /app/target/release/server /app/server

EXPOSE 3000

RUN printf '#!/bin/sh\nset -e\n\nexport HOSTNAME="${HOST:-0.0.0.0}"\n\nif [ "$APP_MODE" = "api-only" ]; then\n    exec ./server\nfi\n\nbun server.js &\nNEXT_PID=$!\ntrap "kill $NEXT_PID 2>/dev/null; exit" TERM INT\nsleep 1\n./server\nkill $NEXT_PID 2>/dev/null\n' > /app/start.sh && chmod +x /app/start.sh

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:3000/api/hello || exit 1

CMD ["/app/start.sh"]
