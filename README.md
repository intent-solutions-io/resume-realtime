# real-time-resume

Real-time collaborative resume builder built with Rust, Leptos SSR + hydration, and Tailwind CSS.

## Quick Start

```bash
# Install dependencies
cargo install cargo-leptos trunk
rustup target add wasm32-unknown-unknown

# Development (SSR + hot reload)
cargo leptos watch

# Production build
cargo leptos build --release
```

## Stack

- **Frontend**: Leptos 0.6+ (SSR + hydration)
- **Backend**: Axum + leptos_axum
- **Styling**: Tailwind CSS v3 (standalone CLI)
- **Build**: Trunk (WASM), cargo-leptos (SSR)

## Development

Server runs at `http://127.0.0.1:3000`

```bash
cargo leptos watch
```

## Project Structure

```
real-time-resume/
├── 000-docs/          # Documentation (flat, no subdirs)
├── src/               # Rust source code
├── styles/            # Tailwind CSS
├── templates/         # HTML templates
├── assets/            # Static assets
└── .github/workflows/ # CI/CD
```

## Documentation

All documentation lives in `000-docs/` following Doc Filing System Standard v3.0. See `CLAUDE.md` for rules.

## License

Proprietary - Intent Solutions IO

---
intent solutions io — confidential IP
Contact: jeremy@intentsolutions.io
