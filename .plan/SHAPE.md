# SHAPE: Rove, a file manager that looks as good as it is fast

## Problem
Flea is the fastest GUI file manager on Omarchy: a 100,000-file folder settles in 0.75 s.
But it is hard to look at, and that is the reason people don't use it.

- **Type:** one monospace font for everything, so there is no hierarchy.
- **Icons:** deliberately blunt, square-capped.
- **Surfaces:** flat navy with 1 px hairlines everywhere and no depth.
- **Theme:** the OS theme paints the whole app, so contrast collapses (text that can't be read on the background) and every app looks the same.

The function list is excellent. The face is the problem.

## Appetite
- **First cycle:** a pixel-accurate, non-functional **grid view with context menu** in Rune, in all four Mode × Material combinations.
- **Then:** list view, menus and dropdowns, settings.
- **Then:** wiring to the backend.

We'd rather ship one screen that is exactly right than six that are approximately right.

## Solution (the shape)
1. **Keep the backend, replace everything above it.**
   - `flea --backend` stays as it is: Rust, JSON-lines over stdin/stdout (`docs/protocol.md` in the Flea repo).
   - The GPUI frontend (`crates/flea-gui`) and the `ui_logic_*` view-model code are replaced by a new Rune app.
2. **Design system: "Calm glass, loud files."**
   - The window stays quiet; content carries the screen.
   - Inter for all reading text. JetBrains Mono only for paths and code. Tabular numbers.
   - Neutral surfaces with no hue, in two **materials**:
     - Glass: translucent, and Hyprland blurs what's behind it.
     - Solid: opaque.
   - Two **modes**, Dark and Light.
   - Layout: the sidebar sits on the window base, and the content lives in a floating rounded panel inset 8 px.
   - Shortcuts are always visible as keycaps.
3. **The Omarchy theme drives accents, never the look.** From the theme's `colors.toml`, Rove reads exactly three things:
   - `mode` (picks Dark or Light),
   - `accent` (selection, menu highlight, focus, progress),
   - the seven tag colours.

   Nothing else. This is a product principle the owner feels strongly about. Do not let more theme colours leak in.
4. **Icons:**
   - Rove's own interface icons: 24 px grid, 1.75 stroke, round caps.
   - File and folder icons come from the user's installed freedesktop icon theme (WhiteSur in the references).
5. **Name:** Rove, after rove beetles, which hunt flea larvae, and because you rove through files. The binary is `rove`.

## Rabbit holes (named so you don't fall in)
- **Blur:**
  - Behind the window, blur is the compositor's job, not ours. On Hyprland, draw a translucent window and Hyprland's `decoration:blur` does the rest.
  - Rune's `system_material` is macOS-only. Don't try to make it work on Linux.
  - Popovers blurring the app's own content use Rune's `El::backdrop_blur`.
  - Headless renders can't show desktop blur. Compare the **Solid** references for exactness.
- **Wayland drag and drop:** Rune can't yet drag files *out* (macOS-only) or receive file drops on Wayland (a winit 0.30 limitation). Not needed for the static milestones. File an issue with Rune (they take issues, not PRs) or carry a patch before Milestone 6.
- **Icon theme lookup:** implement the freedesktop Icon Theme Spec lookup (inherits, sizes, scalable, fallback to `hicolor`). Don't hardcode WhiteSur. For the static milestones, load the bundled reference PNGs from `reference/html/assets/`.
- **Thumbnails:** come from the backend's `thumb` command, scoped to the viewport. Never generate them in the UI thread.
- **Performance:** Flea's headline is speed. Keep its seven load-bearing rules in mind (viewport-scoped work, an integer-count model, first screenful with `list`). Use Rune's `.virtualize()` for list and grid.

## No-gos
- No inspiration from the old Flea UI. Only its features carry over.
- No full-theme painting from Omarchy: no theme backgrounds, no theme foregrounds.
- No monospace UI text.
- No square line caps, no hairline-box controls, no accent left-border cards, no gradients on surfaces.
- No confirmation dialogs for undoable actions. Use Undo toasts. Confirm only what can't be undone (Empty Trash, permanent delete).
- No bundling of GPL icon themes.

## Done means
Your `--snap` render of each milestone's screen matches the reference PNG within the
tolerances in `docs/09-verification.md`, in all four Mode × Material combinations.
