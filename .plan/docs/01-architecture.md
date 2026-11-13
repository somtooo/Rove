# Architecture

## What stays and what goes
The starting point is the Flea repository (github.com/thisisgm/flea, v0.4.2). It has three layers:

| Layer | Path | Fate |
|---|---|---|
| Backend (`flea --backend`): listing, thumbnails, transfers, trash, search, devices, tags, undo… | `src/backend/`, `src/*.rs`, protocol in `docs/protocol.md` | **Keep.** It is the only seam. Rename the binary and command to `rove` when convenient. |
| View-model logic | `src/ui_logic_*` | **Replace.** Read it for behaviour and edge cases (menus, rails, previews), but don't port its structure. |
| GPUI frontend | `crates/flea-gui/` | **Replace** with the Rune app. |
| QML frontend | `ui/` | Already frozen and dead. Ignore. |
| TUI | `src/tui/` | Out of scope. |

## The new crate
```
crates/rove-ui/            Rune app (binary: rove)
  src/main.rs              AppSpec, scenes for headless snaps (see 09-verification.md)
  src/theme.rs             tokens + Omarchy resolution (start from starter/theme.rs)
  src/icons.rs             interface icons (include_str! the SVGs in icons/svg)
  src/icon_theme.rs        freedesktop icon-theme lookup for file and folder icons (Milestone 5)
  src/backend/             child process + JSON-lines client (Milestone 6)
  src/views/
    window.rs              window base, sidebar | panel split
    sidebar.rs             places, tags, network, devices
    toolbar.rs             nav group, title, search, view switch, sort, settings
    tabs.rs                tab bar
    grid.rs                grouped tile grid (virtualized)
    list.rs                list view + header (virtualized)
    footer.rs              breadcrumb, summary, free space, size slider
    menu.rs                ONE menu component: context menu, submenu, dropdown, select
    settings.rs            sheet + pages
  src/mock.rs              the reference data (Milestones 1 to 4)
```
Build every screen from `theme::Tokens`. No literal colours in views.

## Rune notes that matter here
- **Colour:**
  - `rune::Color` is linear sRGB with straight alpha.
  - `rune::hex("#RRGGBB")` decodes sRGB, and `.alpha(a)` sets alpha.
  - Windows blend in sRGB-encoded space by default, like CSS, so keep that default.
- **Text:**
  - Inter 4.1 is embedded.
  - Add JetBrains Mono (OFL) via `AppSpec` fonts for paths and code.
  - Enable tabular figures for numbers.
- **Layout:** Taffy flexbox; the doc values map to `row()/col()`, `.gap()`, `.p()`, `.w()/.h()`, `.radius()`.
  - Mind the three Taffy gotchas in Rune's README: `.grow` needs a bounded parent, use `.min_h(0.0)` on nested columns, and `.virtualize` needs `.growth`.
  - **List rows must not shrink** (`flex: none`). That was a real bug caught in the mockup.
- **Shadows:** `.shadow(y, blur, color)` / `.box_shadow(Shadow)`. Blur follows CSS `box-shadow`, so the token values transfer directly.
- **Blur:**
  - in-app frosted popovers: `El::backdrop_blur(sigma)`, where CSS `blur(28px)` = sigma 14
  - behind the window: the compositor's job (below)
- **Icons:** SVG via `ImageCache::svg(bytes, size_pt, scale)`, or Rune's `Icon` type. Stroke icons must tint to `currentColor`: render each with the token colour.
- **Motion:** `motion::SNAPPY` for popovers and hover retargets, `motion::DEFAULT` for sheets, `motion::CALM` for theme colour changes. Reduced motion means opacity only.
- **Headless agent tooling:** `--snap out.png --scene NAME --size 1440x920 --scale 2 --theme dark|light`, plus `--tree tree.json`. Use these to verify every milestone.

## Glass on Hyprland
1. Create the window transparent. Paint `base` (translucent) as the root fill.
2. Hyprland's blur (`decoration { blur { enabled = true } }`) blurs whatever is behind translucent pixels. Nothing to do in-app.
3. Rove ships a recommended window rule in its docs, e.g. `windowrule = opacity 1.0 override, class:^(rove)$`, so that a global inactive opacity doesn't double up.
4. The **Solid** material is the fallback: no transparency support, a user choice, or when screenshotting.
5. macOS (later): `El::system_material(SystemMaterial::Popover)` in a transparent window.

## Backend protocol: commands you'll use
JSON lines over the child's stdin/stdout. See `docs/protocol.md` in the Flea repo.

| Need | Request → reply |
|---|---|
| Open a folder | `list {path, first, hidden}` → `listed` + `rows` (first screenful rides along) |
| Scroll | `window {start, count}` → `rows` |
| Sort / filter | `sort`, `filter` |
| Search | `search` / `searchquery` / `searchcancel` → `searching`, `searched` |
| Thumbnails | `thumb` / `thumbcancel` → `thumbed` (viewport only) |
| Folder sizes | `dirsize` / `dirsizecancel` → `dirsized` |
| Free space, tag support | `fsinfo` |
| Devices | `devs` |
| Menu actions on a selection | `menuaction` `snapshot` / `activate` / `properties` / `contents` / `facts` / `prepareDelete` / `delete` |
| Copy, move, compress, extract | `transfer`, `collisions`, `mergecollisions`, `transfercancel` |
| Trash | `trash`, `trashbrowse`, `undo` |
| Rename, new folder, duplicate | `rename`, `bulkrename`, `mkdir`, `duplicate` |
| Tags | `settags`, `tagstates` |
| Clipboard | `clipSet`, `clipGet`, `clipClear` |
| Git status | `gitstatus`, `gitrows` |
| Unasked | `changed`: the folder changed on disk; re-list |

Invariants to respect: every request carrying `rows` also carries the `listing` number it read
them from, and stale listings are refused. Errors arrive as messages, not exit codes.
