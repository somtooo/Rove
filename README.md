# Rove

The Rune frontend for a fork of Flea's backend. Design and engineering plan: `.plan/`.

Status: native GUI preview from mock data. Tile selection, hover feedback and the sidebar
toggle work. There is no backend wiring, real navigation or file mutation.

## Layout

- `crates/rove-ui`: the Rune app (binary `rove`). Rune is pinned to `dev` rev `d6a0b3cc`.
  - `src/theme.rs`: tokens and Omarchy theme resolution (ported from `starter/theme.rs`).
  - `src/views/`: window, sidebar, toolbar, tabs, grid, footer; `text.rs` puts labels on Chromium's baselines.
  - `src/mock.rs`: the reference data. `src/assets.rs`: reference icons and thumbnails.
  - `icons/`: Rove's interface icons (MIT). `assets/fonts/`: Inter and JetBrains Mono (OFL).
- `tools/verify.py`: the Milestone 1 checks from `docs/09-verification.md`.
- Rune is fetched by pinned git rev. Separate local Rune and Flea checkouts are references,
  not runtime dependencies. The backend will be forked into this repository later.

## Run

```sh
cargo run -p rove-ui -- --scene dark-glass # floating native window matching the dark reference
cargo test -p rove-ui
python3 -I tools/verify.py               # renders the four scenes and checks them
target/debug/rove --snap out/x.png --scene dark-solid --size 1440x920 --scale 2
target/debug/rove --tree out/x.json --scene dark-solid --size 1440x920 --scale 2
```

Scenes: `dark-glass`, `dark-solid`, `latte-glass`, `latte-solid`. Headless runs draw the window
at (40, 40) on a transparent 1440×920 frame, like the references. `verify.py` composites each
snap over `reference/backdrops/` before comparing.

The icon-theme icons come from the WhiteSur reference PNGs, read from disk at startup (GPL, so
they aren't bundled). Set `ROVE_REFERENCE_ASSETS` to point somewhere else.

Try clicking a file or folder, moving over tiles, or hiding/showing the sidebar (also Ctrl+B).
Esc clears the selection. Other controls remain visual placeholders.

The public repository excludes the third-party reference art and screenshots containing it.
On this development machine the ignored local `.plan/reference/html/assets/` files supply the
preview icons, and the ignored reference screens/backdrop support verification. A fresh clone
does not include those assets. Runtime icon-theme lookup is a later milestone.
