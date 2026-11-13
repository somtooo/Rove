# Rove

The Rune frontend for a fork of Flea's backend. Design and engineering plan: `.plan/`.

Status: native GUI preview from mock data. Tile selection, hover feedback and the sidebar
toggle work. There is no backend wiring, real navigation or file mutation.

## Layout

- `crates/rove-ui`: the Rune app (binary `rove`). Rune is pinned to `dev` rev `d6a0b3cc`.
  - `src/theme.rs`: tokens and Omarchy theme resolution (ported from `starter/theme.rs`).
  - `src/main.rs`: entry point. `src/app/`: startup, shared state and the test harness.
  - `src/components/`: window, sidebar, file grid, toolbar, tabs and footer, each in its own
    directory with its parts and tests. Shared text, icon-button and keycap components live here too.
  - `src/fixtures/`: mock data and reference/review scenes. `src/assets.rs`: image loading.
  - `icons/`: Rove's interface icons (MIT). `assets/fonts/`: Inter and JetBrains Mono (OFL).
- `tools/verify.py`: grid reference checks from `.plan/docs/09-verification.md`.
- Rune is fetched by pinned git rev. Separate local Rune and Flea checkouts are references,
  not runtime dependencies. The backend will be forked into this repository later.

## Development

Work in vertical slices: one usable feature from its UI component through the necessary
Flea code, with end-to-end tests and performance measurements, then commit. The first slice
will show one real directory in the grid. No backend is wired yet.

The current sequence and acceptance checks live in `.plan/docs/08-milestones.md`.
Component boundaries and the backend-fork strategy live in `.plan/docs/01-architecture.md`.
Do not finish every UI screen or import Flea wholesale before starting the first slice.

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

For opacity review, `dark-glass-90` and `latte-glass-90` set only the content panel to 90%
opacity. The original design scenes stay unchanged. Review these alongside `dark-solid`
and `latte-solid` (fully opaque).

The icon-theme icons come from the WhiteSur reference PNGs, read from disk at startup (GPL, so
they aren't bundled). Set `ROVE_REFERENCE_ASSETS` to point somewhere else.

Try clicking a file or folder, moving over tiles, or hiding/showing the sidebar (also Ctrl+B).
Esc clears the selection. Other controls remain visual placeholders.

The public repository excludes the third-party reference art and screenshots containing it.
On this development machine the ignored local `.plan/reference/html/assets/` files supply the
preview icons, and the ignored reference screens/backdrop support verification. A fresh clone
does not include those assets. Runtime icon-theme lookup is a later milestone.
