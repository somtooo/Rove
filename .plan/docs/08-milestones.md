# Milestones

Owner: the lead engineer. Revised 2026-11-13 after reading Flea and Rune
(`01-architecture.md` has the findings). Everything here may change; the design does not.

Each milestone that adds UI ends with headless snaps compared against the references
(`09-verification.md`, run with `python3 -I tools/verify.py`). Do them in order.

**What changed from the first draft, and why**
- **The backend comes in at M3, not M6.** Speed is Flea's headline and the riskiest part of
  Rove (virtualized grid, viewport-scoped requests, first paint). Proving it early beats
  polishing static screens first. Mock scenes stay, so reference checks keep running.
- **Drag and drop gets its own milestone (M8).** Rune can't receive or start file drags on
  Wayland. That is real framework work, not a detail of "Actions".
- **The library split is M9, after measurement.** See `01-architecture.md` › "The backend".
- Menus are drawn in-window by us: Rune has no Linux native menus and rune-kit's `Menu` lacks
  submenu timing, the safe triangle and the design's look.
- The clipboard (copy and cut) goes through Flea's backend clipboard, not Rune's.

## M0: Scaffold ✅ (2026-11-13)
Rune pinned to `dev` `d6a0b3cc`; theme ported; Inter + JetBrains Mono (tabular figures tested);
four scenes.

## M1: Static grid view ✅ (2026-11-13)
Window, sidebar, panel, toolbar, tabs, grouped grid, footer from `src/mock.rs`. All four scenes
pass: 39/39 redline keys within ±1 px, spot colours exact, Solid pixel diff 0.016% / 0.039%.
Labels sit on Chromium's baselines (`views/text.rs`).

## M2: Menus and the keyboard model
- Bump Rune to current `dev` first; `verify.py` must still pass.
- `views/menu.rs`: one data-driven component for the context menu, the Open With submenu,
  dropdowns and selects. Anchored in-window, flipped to stay 8 pt inside, submenu at the
  parent's right edge −6.
- Submenu opens after 120 ms; safe triangle; keyboard (↑/↓, →/←, Enter, Esc, Home/End,
  first-letter jump); outside click closes.
- Glass popovers: `El::backdrop_blur(28.0)` (Rune takes the CSS value).
- Keyboard model: a `keymap.rs` on `flea::keys` (four presets). The grid's 2D cursor in `on_key`
  (Rune's focus groups are 1D). Type-to-search and `/`/`~` path entry as UI states; the Esc
  ladder. Read `10-user-journeys.md` J2–J6 and `11-keybindings.md` first.
- **Done when:** the full `grid-*` references match, menu included, and scripted keyboard runs
  (`--script`) walk the menu and the grid.

## M3: Wire the backend: read-only browsing
- `backend/`: spawn `flea --backend` per pane; a client thread reads JSON lines and hands
  batched events to the UI with `AppHandle::update_if`.
- `list` (first screenful rides along), `window` when the held range drifts, `listing` on every
  guarded request. Grid virtualized as rows of tiles (`virtual_rows`, columns from the width).
- `thumb` for visible tiles only (256 px PNG paths), `thumbcancel` when they scroll away;
  `dirsize`, `fsinfo`, `devs`; `changed` re-lists.
- Navigation, history, tabs; sidebar places from XDG dirs; breadcrumb.
- Icons: the backend sends an icon name per row. Until M6, map names to the reference PNGs and
  a generic fallback.
- **Done when:** browsing real folders works, the mock scenes still pass `verify.py`, and on
  Flea's 100k fixture (`tools/flea-bench`, same machine) Rove's first window and settled times
  are no worse than Flea 0.4.0's GPUI numbers (319 ms / 753 ms, list view). Add a frame-time
  trace (`--trace`) while scrolling: no frame over 16 ms.

## M4: List view, sort dropdown, inspector
- List header and rows (`virtual_rows`, 32 px rows that never shrink), sort dropdown with check
  slots, inspector panel. Real data from M3, plus a mock scene for the references.
- **Done when:** the `list-dark-*` references match and the 100k list stays within M3's numbers.

## M5: Settings sheet
- Scrim, sheet, nav, View page with cards, switches, segmented controls, stepper,
  select + dropdown. Settings read and written through `flea::uistore`.
- **Done when:** the `settings-dark-*` references match and changes persist.

## M6: Live theme and file icons
- Watch `~/.local/state/omarchy/current/theme/colors.toml` (Rove's own watcher; Flea's exists
  but is unused). Resolve per `03-theming.md`; animate with `Tx::color_motion(motion::CALM)`.
- Freedesktop icon-theme lookup for the backend's icon names (start from Flea's
  `openwith_icons.rs`): inherits, sizes, scalable, hicolor fallback. Drop the reference PNGs.
- Hyprland Glass: transparent window, verified on a real session with blur on.
- **Done when:** switching Omarchy themes recolours exactly the accent-driven parts and nothing
  else; every theme in `tokens/omarchy-themes-resolved.json` resolves to its listed accent and ink.

## M7: Actions
- Clipboard through the backend (`clipSet/clipGet/clipWatch`, copy and cut), paste and
  transfer with progress toasts, inline rename, trash + Undo toast, compress and extract, tags,
  Open With, Copy Path, Properties. Reuse `flea::ui_logic_*` view-models where they fit.
- **Done when:** journeys J8–J12 work end to end, each action undoable where the design says so.

## M8: Drag and drop on Wayland
- Rune lacks Wayland file drops in and drags out. Options: file an issue upstream (they take
  issues, not PRs) and/or carry a patch adding `wl_data_device` to Rune's vendored winit.
  Start a spike during M3 to size it; the result decides the route.
- Internal drags (within a window) work already (`on_drag`).
- **Done when:** J9 (drag to copy or move, in and out of Rove) works on Hyprland.

## M9: `flea-core` and the in-process question
- In Flea: move process-wide effects into `main`, explicit `Session::close()`, configured helper
  paths, typed events serialized by the JSON binary.
- Measure an in-process Rove against the child process (first paint, settled, PSS, crash
  behaviour). Switch only on a measured win; keep the child as a fallback.

## Later
Columns view, Two panes, Quick Look, search results, the Trash browser, network and devices
flows, the file-chooser portal, a Properties sheet design, the remaining settings pages. Each
gets its design from the designer first, using this same system.
