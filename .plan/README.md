# Rove: design handoff

Rove is the new name for the Flea file manager. This is a full redesign: a new UI built on
the **Rune** GPU UI framework (github.com/anomalyco/rune), on top of Flea's existing Rust
backend (`flea --backend`, JSON lines).

This folder is everything a coding agent needs to build the design exactly. The colours,
sizes, type and spacing are specified, not suggested.

**To continue building:** use `docs/08-milestones.md` for the current vertical-slice sequence
and `docs/01-architecture.md` for component/backend boundaries. `KICKOFF-PROMPT.md` is the
historical scaffold request; that work is complete.

## Read in this order

| # | File | What it gives you |
|---|---|---|
| 1 | `SHAPE.md` | The pitch: problem, appetite, solution, rabbit holes, no-gos. |
| 2 | `docs/08-milestones.md` | Current vertical-slice plan. **Next: S1, show one real directory in the grid.** |
| 3 | `docs/02-tokens.md` + `tokens/tokens.json` | Every colour, type style, radius and shadow, for Dark/Light × Glass/Solid. |
| 4 | `docs/03-theming.md` | Exactly what the Omarchy theme changes (three things) and what it never changes. |
| 5 | `docs/04-layout.md` + `reference/redlines.json` | Pixel geometry of every region and element, measured from the reference. |
| 6a | `docs/10-user-journeys.md` | **How it feels:** 15 journeys step by step, the click budget, edge cases, and an iteration log of design changes. |
| 6b | `docs/11-keybindings.md` | The complete keyboard map (Default, Vim, Mac, Windows presets). |
| 6 | `docs/screens/grid-view.md` | What every element on the first screen is *for*, its shortcut, and its backend call. |
| 7 | `docs/05-components.md` | Every control and its states. |
| 8 | `docs/06-icons.md` + `icons/` | The interface icon set (61 SVGs) and the rules for file and folder icons. |
| 9 | `docs/07-motion-and-input.md` | Motion, hover and press, focus, keyboard. |
| 10 | `docs/09-verification.md` | How to prove your Rune render matches the reference PNGs. |
| 11 | `docs/01-architecture.md` | How the Rune app talks to the backend; where the Flea code lives. |

## The source of truth

- **`reference/screens/*@2x.png`** shows what it must look like, at 2× scale. Compare your headless Rune render against these.
- **`reference/html/*.html`** is the exact source the PNGs were rendered from. Open one in Chromium and use dev tools to read any value. It is plain HTML/CSS, so every number is inspectable.
- **`reference/redlines.json`** holds measured boxes (x, y, w, h relative to the window) and computed styles for ~80 named elements.
- **`tokens/tokens.json`** holds the token values. `starter/theme.rs` implements them in Rust for Rune.

If a doc and the reference HTML disagree, **the reference HTML wins**. Report the mismatch.

## Reference screens

| File | Mode | Material | Shows |
|---|---|---|---|
| `grid-dark-glass` | Dark (Tokyo Night accent #7C9CFF) | Glass | Grid view, context menu + Open With submenu |
| `grid-dark-solid` | Dark | Solid | Same |
| `grid-latte-glass` | Light (Catppuccin Latte, accent #1E66F5) | Glass | Same, over Latte's wallpaper |
| `grid-latte-solid` | Light | Solid | Same |
| `grid-anatomy` | — | — | Grid view with 33 numbered pins (legend in `docs/screens/grid-view.md`) |
| `list-dark-glass` / `-solid` | Dark | both | List view, sort dropdown, inspector panel |
| `settings-dark-glass` / `-solid` | Dark | both | Settings sheet, View page, select dropdown open |
| `tabs-journey` | — | — | Storyboard: tab behaviour, step by step, with click costs |
| `search-journey` | — | — | Storyboard: type-to-search and path entry |
| `foundations` | — | — | The design system: materials, colour, type, space, radius, depth, icons |
| `components` | — | — | Every component in its states |

The live design canvas (Claude Design) holds the same artboards and is the editable version.

## Licences

- `icons/` is Rove's own icon set (MIT, same as the project).
- `reference/html/assets/whitesur-*.png` are renders of the WhiteSur icon theme (GPL-3.0), for reference only. **Do not bundle them.** Rove reads file and folder icons from the user's installed icon theme at runtime.
- `catppuccin-latte-wallpaper.jpg` comes from Omarchy's theme and is for reference only.
