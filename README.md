# Herblytics

```
██╗  ██╗███████╗██████╗ ██████╗ ██╗     ██╗   ██╗████████╗██╗ ██████╗ ██████╗
██║  ██║██╔════╝██╔══██╗██╔══██╗██║     ╚██╗ ██╔╝╚══██╔══╝██║██╔════╝██╔════╝
███████║█████╗  ██████╔╝██████╔╝██║      ╚████╔╝    ██║   ██║██║     ███████╗
██║  ██║██╔══╝  ██╔══██╗██╔══██╗██║       ╚═══╝     ██║   ██║██║     ╚════██║
██║  ██║███████╗██║  ██║██████╔╝███████╗  ██╗       ██║   ██║╚██████╗██████╔╝
╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═════╝╚══════╝  ╚═╝   ╚═╝╚═╝ ╚═════╝╚═════╝
```

---

## ◆ PULSE

A purchasing decision deserves a number, not a hunch. Herblytics is the
dashboard that turns herbal medicine procurement at Sabot Hospital into
one Thai fiscal year of visible truth: total annual purchase value,
the top ten products, and the trends that a budget meeting actually
needs. Compiled to WASM with Leptos, drawn as self-contained SVG with no
charting library in sight, served by a Google Apps Script that costs
nothing to maintain - the insight is the product, the plumbing is
boring on purpose.

| v1.1.17 ▣ | SVG chart ▣ | Fiscal year ▣ | Apps Script ▣ |
|---|---|---|---|

*The dashboard - summary, ranking, trends - is sealed and shipping.*

> Built with Rust 2024 + Leptos 0.8, bundled by Trunk, styled by
> Tailwind 4, answered by one Google Apps Script endpoint.
>
> **suradet-ps**, artifact keeper

---

## ◆ IGNITION

Two terminals, one API key.

```
⟫ bun install
⟫ make dev
```

Tailwind watches `src/**/*.rs`; Trunk serves on
[http://127.0.0.1:3000](http://127.0.0.1:3000).

Point at the data: the Apps Script URL is baked in at build time -

```
⟫ GOOGLE_API_URL="https://script.google.com/macros/s/XXXX/exec" trunk serve
```

or, without rebuilding, set `herb_lytics_api_url` in the browser
console's `localStorage`.

<details>
<summary>Prerequisites</summary>

| Tool | Version | Note |
|---|---|---|
| Rust | 1.88+ | with the `wasm32-unknown-unknown` target |
| Trunk | 0.21+ | `cargo install trunk --locked` |
| Bun | 1.0+ | compiles Tailwind CSS only |

</details>

---

## ◆ ANATOMY

One screen, four honest parts, no charting dependency.

- **Summarizes** - the summary cards state the fiscal year at a glance:
  total purchase value and the metrics a pharmacy budget lives by.
- **Ranks** - a self-contained SVG bar chart draws the top ten herbal
  medicines by purchase value - pure markup, no JS charting library,
  nothing to break between releases.
- **Answers** - the Google Apps Script backend serves one question per
  year (`?path=getHerbSummary&year={year}`), and the `{ status, data }`
  envelope is parsed and validated by `serde` in `core/api.rs` - no
  silent shape drift from the spreadsheet side.
- **Remembers** - an `OnceLock` store of `RwSignal`s replaces Pinia: a
  `Copy` struct of signals is the whole state, with no store ceremony.
- **Migrates** - the road from Vue to Rust is documented end to end in
  `MIGRATION.md`; every replacement (Pinia -> OnceLock, Zod -> serde,
  Vite -> Trunk) is named and justified.

---

## ◆ RITUALS

**The core ceremony** - the quarterly budget glance:

1. Open the dashboard. The fiscal year's summary answers first.
2. Read the top ten - the bar chart tells the ranking without a legend
   lecture.
3. Switch the year; the same question, the same shape, a new answer.
4. Decide with a number in hand - the dashboard is done; the meeting
   can begin.

**The ceremony of the envelope** - the API either answers or says it
cannot: `status`, `message`, `data`. A malformed payload is caught at
the boundary, never rendered as a plausible-looking lie.

**The ceremony of restraint** - no charting library, no state
framework, no server to feed. Each dependency carries its weight or is
replaced - the migration log proves the discipline.

---

## ◆ ECHOES

**Where this artifact is heading**

```
summarize ▸ fiscal-year summary cards ─────────────────────────────── ▸ sealed
rank      ▸ top-10 SVG bar chart, self-contained ──────────────────── ▸ sealed
answer    ▸ Apps Script endpoint + serde-validated envelope ────────── ▸ sealed
migrate   ▸ Vue-to-Rust path documented in MIGRATION.md ────────────── ▸ sealed
```

**Raising the artifact** - the envelope and types live in `core/`; the
CSS pipeline in `tailwind.css`; the version history in `CHANGELOG.md`.
Gates before any PR: `make fmt`, `make clippy`, `make test`. Open an
issue first to discuss a change.

**Status** - CI runs the Rust + Leptos gates on every push.
[Watch the gates](.github/workflows).

---

```
  ─────────────────────────────────────────
   The top ten is not a ranking.
   It is the budget speaking in numbers.
  ─────────────────────────────────────────
```

Licensed under the [MIT License](LICENSE).