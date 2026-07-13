# Herblytics

![CI](https://img.shields.io/badge/CI-Rust%20%2F%20Leptos-DEA584?style=flat-square)
[![Rust](https://img.shields.io/badge/Rust-1.88+-000000?logo=rust)](https://www.rust-lang.org/)
[![Leptos](https://img.shields.io/badge/Leptos-0.8-4FC08D)](https://leptos.dev/)
[![Trunk](https://img.shields.io/badge/Trunk-0.21-646CFF)](https://trunkrs.dev/)
[![Tailwind CSS](https://img.shields.io/badge/Tailwind_CSS-4.1-06B6D4?logo=tailwindcss)](https://tailwindcss.com/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> **A data visualization dashboard for tracking and analyzing the procurement value of herbal medicines at Sabot Hospital.**

This application provides actionable insights into purchasing trends, helping hospital staff and administrators identify key metrics such as total annual purchase value and top-performing herbal products.

The frontend is compiled to **WebAssembly** with [Leptos](https://leptos.dev) (CSR) and bundled by [Trunk](https://trunkrs.dev). See [`MIGRATION.md`](./MIGRATION.md) for the full Vue → Rust migration guide.

---

## Features

### Data Visualization

- **Interactive Charts**: A self-contained **SVG** bar chart (no JS charting dependency) shows the top 10 herbal medicines by total purchase value.
- **Dynamic Summaries**: Summary cards displaying high-level metrics for the selected Thai fiscal year.

### Core Stack

- **[Rust 1.88+](https://www.rust-lang.org/)** (edition 2024) compiled to `wasm32-unknown-unknown`.
- **[Leptos 0.8](https://leptos.dev/)** in CSR mode — `#[component]` functions instead of Vue SFCs.
- **[Trunk](https://trunkrs.dev/)** as the bundler/dev server (replaces Vite).
- **[Tailwind CSS 4](https://tailwindcss.com/)**: compiled to plain CSS via the CLI; class names are scanned from `src/**/*.rs`.

### Architecture

- **OnceLock store**: a `Copy` struct of `RwSignal`s (`stores/dashboard.rs`) replaces Pinia.
- **Google Apps Script backend**: a lightweight, maintenance-free backend serving data via `?path=getHerbSummary&year={year}`.
- **`serde` validation**: the API envelope (`{ status, message?, data? }`) is parsed and validated in `core/api.rs`, replacing Zod.

---

## Prerequisites

| Requirement | Version | Note |
| :---------- | :------ | :-- |
| **Rust**    | `1.88+` | With the `wasm32-unknown-unknown` target (`rustup target add wasm32-unknown-unknown`). |
| **Trunk**   | `0.21+` | `cargo install trunk --locked`. |
| **Bun**     | `1.0+`  | Only used to compile Tailwind CSS (lockfile present). |

---

## Getting Started

```bash
# 1. Clone
git clone https://github.com/suradet-ps/herb-lytics.git
cd herb-lytics

# 2. Compile Tailwind CSS (watches src/**/*.rs in dev)
bun install
bun run watch:css      # or: make watch

# 3. Start the dev server (in another terminal)
make dev               # runs tailwind --watch + trunk serve
# or simply:
trunk serve
```

The app is served at `http://127.0.0.1:3000/`.

### Pointing at the API

The Google Apps Script URL is read at build time from `GOOGLE_API_URL`:

```bash
GOOGLE_API_URL="https://script.google.com/macros/s/XXXX/exec" trunk serve
```

For local development without rebuilding, set the `herb_lytics_api_url`
key in `localStorage` from the browser console.

---

## Available Scripts

| Command              | Description |
| :------------------- | :---------- |
| `make dev`           | Tailwind watch + `trunk serve`. |
| `make build`         | `bun run build:css` then `trunk build --release`. |
| `make check`         | `cargo check --target wasm32-unknown-unknown`. |
| `make clippy`        | `cargo clippy` (wasm) with correctness/suspicious denied. |
| `make fmt`           | `cargo fmt --all --check`. |
| `make test`          | `cargo test --lib`. |

---

## Project Structure

```text
.
├── src/
│   ├── core/           # Pure logic: error, config, types, api, time, utils
│   ├── stores/         # OnceLock singleton state (dashboard)
│   ├── components/      # Leptos #[component]s (layout, cards, chart, table)
│   ├── views/          # Page-level view (dashboard)
│   ├── app.rs          # Root component + <meta> context
│   └── lib.rs          # wasm-bindgen entry point
├── public/
│   └── styles/         # Tailwind-generated main.css (git-ignored, built from tailwind.css)
├── tailwind.css        # Theme tokens + @source "./src/**/*.rs"
├── Cargo.toml          # Leptos 0.8 CSR deps
├── Trunk.toml          # Trunk config
├── rust-toolchain.toml # stable + wasm32 target
├── Makefile            # dev / build / check wrappers
└── vercel.json         # Static SPA deploy (dist/)
```

---

## Deployment

### Vercel (static SPA)

- **Build Command**: `bun install --frozen-lockfile && bun run build:css && rustup target add wasm32-unknown-unknown && cargo install trunk --locked && trunk build --release`
- **Output Directory**: `dist`
- **Environment Variable**: set `GOOGLE_API_URL` as a build-time var so it is baked into the WASM.
- Deep links use an SPA rewrite to `index.html`; `.wasm` is served as `application/wasm`.

---

## Contributing

1. **Fork** the repository.
2. **Create** a feature branch: `git checkout -b feat/my-feature`.
3. **Commit** your changes: `git commit -m "feat: add amazing feature"`.
4. **Push** to the branch: `git push origin feat/my-feature`.
5. **Open** a Pull Request.

---

## License

This project is licensed under the [MIT License](LICENSE).
