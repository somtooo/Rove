# Starter code

- `theme.rs`: Rove tokens and Omarchy theme resolution for Rune. A reference implementation:
  **compile-check it against your pinned Rune rev first** (it was written against Rune `dev`,
  2026-11-13, and not compiled in the design session). The values must equal `tokens/tokens.json`.
- Interface icons: `include_str!("../../icons/svg/<name>.svg")` and render through Rune's SVG
  image cache at the token colour (replace `currentColor` before rasterising, or tint).
