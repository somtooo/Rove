# Milestones

Each milestone ends with headless snaps compared against the references (`09-verification.md`).
Do them in order. Don't wire the backend before Milestone 6.

## M0: Scaffold (half a day)
- New crate `crates/rove-ui` in the Flea repo (or a fresh repo that vendors the backend later). Pin Rune to a `dev` rev.
- Port `starter/theme.rs`. Compile-check it against the pinned Rune and fix any API drift.
- Load JetBrains Mono. Confirm Inter renders with tabular figures.
- Add scenes: `dark-glass`, `dark-solid`, `latte-glass`, `latte-solid`. Each sets `Tokens` from a fixed `OmarchyTheme` (Tokyo Night accent `#7C9CFF` dark; Latte `#1e66f5` light).
- **Done when:** `rove --snap out.png --scene dark-solid --size 1440x920 --scale 2` produces a PNG of an empty 1360×840 window base at (40, 40) over a backdrop.

## M1: Static grid view (the first real milestone)
- Build window, sidebar, panel, toolbar, tabs bar, grouped grid and footer from `src/mock.rs`, which reproduces exactly the reference data: Documents, 6 folders, 10 files, `shore-at-dusk.jpg` selected, `forest-walk.jpg` hovered.
- Icons: the interface SVGs from `icons/svg`. For theme icons, load the PNGs in `reference/html/assets/` directly (the icon-theme lookup comes in M5).
- No context menu yet.
- **Done when:** the snap of each scene matches `grid-*-solid@2x.png` with the menu region excluded (crop or mask x 572–1058, y 364–782 window-relative). Glass is checked by eye over a backdrop.

## M2: Menus and the keyboard model
- Before starting, read `10-user-journeys.md` (J2–J6) and `11-keybindings.md`. Type-to-search, path entry and the Esc ladder land in this milestone.
- One `menu.rs` component; data-driven items; context menu + Open With submenu, exactly as the reference.
- Keyboard navigation, submenu timing and safe triangle, placement and flipping, Esc and outside-click.
- Popover backdrop blur in Glass (`El::backdrop_blur(14.0)`).
- **Done when:** the full `grid-*` references match, including the menu.

## M3: List view and sort dropdown, inspector
- List header + rows (virtualized, 32 px rows), sort dropdown with check slots, inspector panel.
- **Done when:** the `list-dark-*` references match.

## M4: Settings sheet
- Scrim, sheet, nav, View page with cards, switches, segmented controls, stepper, select + dropdown.
- **Done when:** the `settings-dark-*` references match.

## M5: Live theme and icons
- Read and watch `~/.local/state/omarchy/current/theme/colors.toml` (reuse Flea's `src/theme/watch.rs`). Resolve per `03-theming.md`. Animate theme switches.
- Freedesktop icon-theme lookup and MIME mapping.
- Hyprland Glass: transparent window; verify on a real session with blur on.
- **Done when:** switching Omarchy themes recolours exactly the accent-driven parts listed in `03-theming.md` and nothing else; every theme in `tokens/omarchy-themes-resolved.json` resolves to the listed accent and ink.

## M6: Wire the backend
- Spawn `flea --backend` per pane; JSON-lines client off the UI thread.
- `list` / `rows` / `window` into a virtualized grid and list; `thumb` for visible tiles; `fsinfo`, `devs`; navigation, history and tabs.
- Respect Flea's load-bearing rules: viewport-scoped work, the first screenful riding along with `list`, `listing` numbers on every row request.
- **Done when:** browsing real folders works, and a 100k-file folder stays smooth (compare with Flea's bench in `docs/bench`).

## M7: Actions
- Clipboard, paste and transfer with progress toasts, rename (inline), trash + Undo toast, compress and extract, tags, Open With (desktop entries), Copy Path, Properties.
- Solve the Wayland drag-and-drop gaps in Rune (an issue upstream, or a patch).

## Later
Columns view, Two panes, Quick Look, search results, the Trash browser, network and devices flows,
the file-chooser portal, a Properties sheet design, the remaining settings pages. Each gets its design on the
canvas first, using this same system.
