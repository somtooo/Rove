# Rove

The Rune frontend for the Flea file manager's backend. Design handoff: `rove-handoff/`.

Status: Milestones 0 and 1, a static, non-functional grid view from mock data.

## Layout

- `crates/rove-ui`: the Rune app (binary `rove`). Rune is pinned to `dev` rev `d6a0b3cc`.
  - `src/theme.rs`: tokens and Omarchy theme resolution (ported from `starter/theme.rs`).
  - `src/views/`: window, sidebar, toolbar, tabs, grid, footer; `text.rs` puts labels on Chromium's baselines.
  - `src/mock.rs`: the reference data. `src/assets.rs`: reference icons and thumbnails.
  - `icons/`: Rove's interface icons (MIT). `assets/fonts/`: Inter and JetBrains Mono (OFL).
- `tools/verify.py`: the Milestone 1 checks from `docs/09-verification.md`.
- `vendor/rune`: a read-only clone of Rune for reading its docs. The build fetches Rune by git rev.

## Run

```sh
cargo run -p rove-ui                     # a window (Glass, colours from the current Omarchy theme)
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
