# Architecture

Owner: the lead engineer. Verified against Flea `5da91810` and Rune `dev` on 2026-11-13
(sources: `~/devcave/flea`, `~/devcave/rune`). Where this doc and the handoff's first draft
disagreed, the code won; the corrections are listed at the end.

## The shape

**Owner decisions, 2026-11-13:** fork the backend and required shared code into this Rove
repository. Do not depend on or modify the local Flea checkout; keep that for reference.
Rove owns `$XDG_STATE_HOME/rove/ui.json`, not Flea's state. The next deliverable is the
native mock GUI; backend integration waits. The earlier study below describes the original
Flea boundaries and process-isolation tradeoffs, not a decision to depend on installed Flea.

```
Rove (this repo)
  crates/rove-ui        Rune app, binary `rove`
    mock data only      ← current GUI preview, no backend linked or launched
    planned:
      in-repo backend fork + required shared helpers from Flea
      child-process transport first; library/session extraction considered separately
      state at $XDG_STATE_HOME/rove/ui.json
```

## What Rove takes from Flea, and how

Flea is one Rust package with a library (`src/lib.rs`) and a binary (`src/main.rs`). The
library has no GUI dependency (only foldhash, regex, unicode crates). Its useful modules can
be retained in the in-repo fork rather than copied into UI code or pulled from upstream at runtime.

| Need | Flea source | Library or binary? | Rove's use |
|---|---|---|---|
| Listing, thumbnails, search, transfers, trash, tags, devices, mounts | `src/backend/` (~87k non-test lines) | binary only | spawn `flea --backend` |
| Protocol types | `src/protocol.rs` (client side: only `list`, `window`, `quit`) | library | extend, or write Rove's own typed client |
| Keymap with Default/Vim/Mac/Windows presets | `src/keys.rs`, `keys.toml`, generated `keys_generated.rs` | library | use directly; update `keys.toml` to `11-keybindings.md` |
| Settings and session state (`$XDG_STATE_HOME/flea/ui.json`) | `src/uistore.rs`, `src/uistate.rs` | library | use the same store format (decision on sharing the file: see below) |
| Omarchy `colors.toml` parsing | `src/theme/` | library | parse; the watcher exists but **nothing calls it**, so Rove writes its own watch |
| View-model logic: menus, file ops, rail places, device rail, Open With, settings pages, previews… | `src/ui_logic_*` (25 modules, ~67k non-test lines) | library, no GUI deps | **reuse instead of re-implementing**, module by module, after checking each one's I/O |
| Open, default apps | `src/open.rs` (delegates to `gio open`), `src/desktopid.rs`, `src/defaults.rs` | binary only | through the backend, or move into the library |
| MIME type and icon name per file | backend (`mime.rs` reads shared-mime-info; rows carry an icon *name*) | binary only | arrives on the wire; Rove resolves the name to a file |
| Icon-theme lookup (index.theme, inherits, hicolor) | `ui_logic_open_with_properties/openwith_icons.rs` (app icons only) | library | starting point for Rove's file-icon lookup |
| File clipboard on Wayland (copy *and* cut) | `src/clip/` (std-only wl data-control client) via `clipSet/clipGet/clipClear/clipWatch` | binary | use through the backend. This covers Rune's gap (Rune's clipboard can't do cut) |

## The backend: child process first, library later

The owner wants Flea's backend as a library ("libghostty for file managers"). The study says
yes, eventually, but not as the first step, and not for raw speed.

**What the wire actually costs** (Flea's own numbers): spawn to first `listed` is 3.1–3.4 ms;
a 300-row `rows` reply takes about 0.9 ms to produce and about 0.9 ms to parse. The big cost was
round trips, and protocol rule 3 already removed it (first screenful rides along with `list`:
4 ms instead of 64 ms). Nobody has measured the hop in isolation. An in-process API saves
roughly a millisecond per screenful; it doesn't change the headline numbers.

**What in-process costs us today:**
- `let_go_of_client()` points fds 0 and 1 at `/dev/null` (`run.rs:1010`): it would silence Rove.
- `pin_mmap_threshold()` changes glibc's allocator for the whole process (`heap.rs:16`), and the
  file-descriptor limit is raised process-wide (`picker_limits.rs:45`).
- Workers re-launch `current_exe()` (3D helper, plugin shelf, archive, locale, video worker): in
  Rove that is Rove.
- Shutdown is stdin EOF, and stuck mount threads are cleaned up by `exit()`.
- About 250 `unsafe` blocks: a crash today kills only the child; in-process it kills the window.

**What a library is good for:** typed requests and events instead of hand-written JSON, one
build, one process to profile, and other frontends (the TUI, a portal) on the same core.

**Plan:**
1. No backend work in the current GUI preview. When integration begins, run the in-repo fork
   as a child and speak JSON lines from a client thread. No installed Flea dependency.
2. M9 (`flea-core`): in Rove's fork, move the process-wide effects into `main`, give the session an
   explicit `close()`, pass helper paths in config, and make workers emit typed `Event`s that
   `protoreply.rs` then serializes. The existing seams (`Event` channel, generic `Write`,
   fd-based inbox) make this mostly mechanical. The JSON binary stays as is on top.
3. Then measure an in-process Rove against the child (first paint, settled, PSS). Switch only if
   it wins. Keep the child as a fallback either way.

Where performance really is to be found: the frontend. Flea's GPUI build settles a 100k folder
in 753 ms and maps its first window in 319 ms (`docs/bench/0.4.0-20261102.md:89`). The backend
part of that is a few ms. Rove's job is to beat the frontend side.

## Rune: what we can and can't rely on

| Need | Rune today | Consequence |
|---|---|---|
| 100k-row list | `virtual_rows(count, est, key, build)` builds mounted rows only; keys are compared for every row each commit, so set `.virtual_rows_version(v)` | fine |
| Grid | no 2D virtualizer | virtualize **rows of tiles**; column count from the width |
| Menus | no native menus on Linux; in-window `El::anchored` flips and stays 8 pt inside the window; rune-kit `Menu` has no submenu delay or safe triangle and the wrong look | our own `menu.rs`, drawn in-window (as the design specifies) |
| Arrow keys in a grid | `focus_group` moves in flat order only | our own 2D cursor in `on_key` |
| Popover blur | `El::backdrop_blur(sigma)` takes the CSS `blur()` value directly (so 28, not 14); max 4 per frame | fine |
| Glass | transparent windows work; Hyprland blurs translucent windows itself | fine; verify on a real session |
| Drag files **in** on Wayland | **no** (`on_drop_files` is macOS, Windows, X11; winit 0.30 has no `wl_data_device`) | blocker for drag-and-drop journeys; see M8 |
| Drag files **out** on Linux | **no** (macOS and Windows only) | same |
| Clipboard | text and file copy via arboard; **no cut** | use Flea's backend clipboard instead |
| OS notifications | `rune-notify` is a stub on Linux | not needed: toasts are in-app |
| Events from a backend thread | `AppHandle::update_if(|s, tx| …)`, batched; `AppSpec::setup` is a plain `fn`, so pass channels through `App::init` | client thread → batched updates |
| Accessibility | AccessKit/AT-SPI on by default | free; name our elements |
| API stability | version 0.0.1, 2,300+ commits in three weeks | pin a rev, bump deliberately, rerun `tools/verify.py` on every bump |

## Crate layout (current and planned)

```
crates/rove-ui/
  src/main.rs           AppSpec, scenes
  src/theme.rs          tokens + Omarchy resolution
  src/icons.rs          interface icons
  src/assets.rs         reference images (M1–M4); replaced by icon_theme.rs + thumbnails
  src/mock.rs           reference data, kept for scenes and verification after M3
  src/views/            window, sidebar, toolbar, tabs, grid, footer, text (Chromium baselines)
  planned: views/menu.rs, views/list.rs, views/settings.rs, keymap.rs,
           backend/ (client thread, protocol), icon_theme.rs, theme_watch.rs
```
Every screen is built from `theme::Tokens`. No literal colours in views. Mock scenes stay
available after the backend lands, so the reference checks keep running.

## Settled decisions (owner)
1. **Backend ownership:** fork into the Rove repo. Any keymap changes and library extraction
   happen in that fork, not in upstream or the reference checkout.
2. **Separate state:** `$XDG_STATE_HOME/rove/ui.json`, using compatible helpers where useful.
   Audit hardcoded Flea cache, config and state paths before enabling the backend.
3. **Public repository:** third-party GPL art and reference screenshots containing it stay
   local and ignored. They are not public-repository or runtime-binary assets.
4. **Immediate scope:** native GUI with mock data. No backend wiring during the preview.

## Corrections to the handoff's first draft
- "Every request carrying rows carries `listing`; stale ones are refused." Rows *replies* are
  always stamped. On requests the field is optional, and only nine file-acting commands refuse
  a stale one (`rowguard.rs:8`); read-only requests (`thumb`, `dirsize`, `window`…) never are.
- "Flea's `src/theme/watch.rs` already watches colors.toml." It exists (polling) but nothing
  calls it; flea-gui reads the file once.
- "`src/open.rs`: xdg mimeapps default." It delegates to `gio open`; defaults are read via
  `gio mime` and written via `xdg-mime` (`defaults.rs`).
- "Flea does icon-theme lookup." For app icons only. File rows carry an icon name from
  shared-mime-info; resolving it to a file is Rove's job.
- The protocol table lists about half the commands. `docs/protocol.md` in Flea is the reference,
  and it too misses `peek`, `paths`, `archive`, `convert`, `formats`.
- Rune `backdrop_blur` takes the CSS value (28), not σ = 14.
