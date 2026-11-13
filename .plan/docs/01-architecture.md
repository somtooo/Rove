# Architecture

Owner: the lead engineer. Findings verified against Flea `5da91810` and Rune `dev` on 2026-11-13
(sources: `~/devcave/flea`, `~/devcave/rune`); delivery plan revised with som2 on the same date.
Where this doc and the handoff's first draft
disagreed, the code won; the corrections are listed at the end.

## The shape

**Owner decisions, 2026-11-13:** fork the backend and required shared code into this Rove
repository. Do not depend on or modify the local Flea checkout; keep that for reference.
Rove owns `$XDG_STATE_HOME/rove/ui.json`, not Flea's state. The native mock preview is working.
From here, deliver features as vertical slices: the owning component, only the Flea code
that feature needs, end-to-end tests and measured performance, then a working commit.
`08-milestones.md` is the current sequence. No backend is wired in this preparation commit.
The study below describes Flea's boundaries and process-isolation tradeoffs, not a decision
to depend on installed Flea or import its entire backend in advance.

```
Rove (this repo)
  crates/rove-ui        Rune app, binary `rove`
    mock data only      ← current GUI preview, no backend linked or launched
    planned:
      in-repo backend fork + required shared helpers from Flea
      child-process transport first; library/session extraction considered separately
      state at $XDG_STATE_HOME/rove/ui.json

Sibling workspace checkouts (not runtime dependencies)
  ~/devcave/rune        framework reference; targeted patches only when a slice needs them
  ~/devcave/flea        read-only source reference for incremental extraction into Rove
```

## What Rove takes from Flea, and how

Flea is one Rust package with a library (`src/lib.rs`) and a binary (`src/main.rs`). The
library has no GUI dependency (only foldhash, regex, unicode crates). Its useful modules can
be retained in the in-repo fork rather than copied into UI code or pulled from upstream at runtime.

| Need | Flea source | Library or binary? | Rove's use |
|---|---|---|---|
| Listing, thumbnails, search, transfers, trash, tags, devices, mounts | `src/backend/` (~87k non-test lines) | binary only | spawn `flea --backend` |
| Protocol types | `src/protocol.rs` (client side: only `list`, `window`, `quit`) | library | extend, or write Rove's own typed client |
| Keymap with Default/Vim/Mac/Windows presets | `src/keys.rs`, `keys.toml`, generated `keys_generated.rs` | library | retain in Rove's fork when needed; adapt to `11-keybindings.md` |
| Settings and session state (`$XDG_STATE_HOME/flea/ui.json`) | `src/uistore.rs`, `src/uistate.rs` | library | compatible format, separate Rove path; audit hardcoded defaults |
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
serialization/parsing work, but no isolated benchmark yet establishes Rove's net gain.

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
1. The first real-directory slice brings in the listing path and its required dependencies.
   Run the in-repo fork as a child and speak JSON lines from a client thread. No installed
   Flea dependency, speculative full import, or backend I/O inside component rendering.
2. If a slice or profile justifies library extraction, in Rove's fork move process-wide
   effects into `main`, give the session an explicit `close()`, pass helper paths in config,
   and make workers emit typed `Event`s that
   `protoreply.rs` then serializes. The existing seams (`Event` channel, generic `Write`,
   fd-based inbox) are candidates to audit, not proof extraction is trivial. Keep the
   JSON binary as a process adapter on top.
3. Then measure an in-process Rove against the child (first paint, settled, PSS). Switch only if
   it wins. Keep the child as a fallback either way.

The frontend needs its own measurements. Flea's GPUI build settles a 100k folder
in 753 ms and maps its first window in 319 ms (`docs/bench/0.4.0-20261102.md:89`). The backend
part of that is a few ms in that benchmark. Those figures do not prove Rove or Rune will
be fast. Design for bounded visible work and measure backend, transport, UI building,
layout and rendering separately on the same machine. See the per-slice performance gate
in `08-milestones.md`.

## Rune: what we can and can't rely on

| Need | Rune today | Consequence |
|---|---|---|
| 100k-row list | `virtual_rows(count, est, key, build)` builds mounted rows only; keys are compared for every row each commit, so set `.virtual_rows_version(v)` | fine |
| Grid | no 2D virtualizer | virtualize **rows of tiles**; column count from the width |
| Menus | no native menus on Linux; in-window `El::anchored` flips and stays 8 pt inside the window; rune-kit `Menu` has no submenu delay or safe triangle and the wrong look | a reusable in-window menu component, added with the first menu-backed feature |
| Arrow keys in a grid | `focus_group` moves in flat order only | our own 2D cursor in `on_key` |
| Popover blur | `El::backdrop_blur(sigma)` takes the CSS `blur()` value directly (so 28, not 14); max 4 per frame | fine |
| Glass | transparent windows work; Hyprland blurs translucent windows itself | fine; verify on a real session |
| Drag files **in** on Wayland | **no** (`on_drop_files` is macOS, Windows, X11; winit 0.30 has no `wl_data_device`) | framework work within the drag-file slice |
| Drag files **out** on Linux | **no** (macOS and Windows only) | same |
| Clipboard | text and file copy via arboard; **no cut** | use Flea's backend clipboard instead |
| OS notifications | `rune-notify` is a stub on Linux | not needed: toasts are in-app |
| Events from a backend thread | `AppHandle::update_if(|s, tx| …)`, batched; `AppSpec::setup` is a plain `fn`, so pass channels through `App::init` | client thread → batched updates |
| Accessibility | AccessKit/AT-SPI on by default | free; name our elements |
| API stability | version 0.0.1, 2,300+ commits in three weeks | pin a rev, bump deliberately, rerun `tools/verify.py` on every bump |

## Component-based UI crate

```
crates/rove-ui/
  assets/fonts/         licensed font files, outside src
  icons/                MIT interface SVGs, outside src
  src/
    main.rs             entry point only
    app/
      mod.rs            AppSpec, fonts, startup, current shared State, global shortcuts
      test_support.rs   shared runtime harness, test-only
      tests.rs          app/font tests
    components/
      mod.rs            component declarations and shared-component exports
      window/           composition of sidebar + floating panel; layout tests
      sidebar/          brand, places, tags, devices; sidebar tests
      file_grid/
        mod.rs          grouped grid and bounded viewport
        group_header.rs
        tile.rs         artwork, labels, badge and pointer behaviour
        tests.rs        selection, clipping and scrolling
      toolbar/          navigation, title/count, search, view switch, sort, settings button
      tabs/             tab strip and tab chips
      footer/           breadcrumbs, selection summary, free space, size slider
      icon_button.rs    shared icon-button component
      keycap.rs         shared keyboard-shortcut component
      text.rs           shared labels and Chromium-baseline calibration/tests
      element_key.rs    stable keys for redlines and tests, not UI
    fixtures/
      mod.rs            fixed mock data used by the current preview
      scenes.rs         reference themes and 90%-panel review variants
    assets.rs           image loading/reference-art preparation, not component rendering
    icons.rs            interface icon definitions
    theme.rs            tokens + Omarchy resolution
```

Every major component has one directory. Its `mod.rs` is Rust's directory entry point,
not an extra UI layer. Private parts, state and tests live with the component; split
files only when there is a real responsibility to name. Small shared primitives stay
single files. Do not add empty folders for unimplemented components.

`window` composes the component tree. Components read state and emit user intent; they
must not perform filesystem I/O, start processes or decode assets while rendering.
The current mock preview still uses shared `app::State` and fixture-specific entry types.
As each real slice arrives, move its model/state next to the owning component and keep
only genuinely shared session data in `app`; do not build a speculative global store.
Preserve the existing Rune element keys so geometry and interaction tests survive moves.

All UI colours come from `theme::Tokens`. Mock scenes stay available after real data
lands, so reference checks remain deterministic and never enumerate the owner's files.
The 90% Glass variants are review overrides, not changes to the designer's original tokens.

The first backend-backed slice adds `crates/rove-backend/` and a narrow UI client boundary
for listing. Extend that fork/client in later slices, with provenance and notices beside
the imported source. Icon-theme lookup, thumbnails, keymaps, menus and settings storage
arrive with their owning features, not as parallel infrastructure projects.

Rune remains pinned. A local framework checkout is for reading and reproducing issues;
if a measured bottleneck or missing capability requires a patch, document the reproducer
and before/after results, test it there, then record a reproducible dependency revision or
patch in Rove. Do not commit a machine-local `path` dependency as the production build.

## Settled decisions (owner)
1. **Backend ownership:** fork into the Rove repo. Any keymap changes and library extraction
   happen in that fork, not in upstream or the reference checkout.
2. **Separate state:** `$XDG_STATE_HOME/rove/ui.json`, using compatible helpers where useful.
   Audit hardcoded Flea cache, config and state paths before enabling the backend.
3. **Public repository:** third-party GPL art and reference screenshots containing it stay
   local and ignored. They are not public-repository or runtime-binary assets.
4. **Delivery model:** one usable vertical slice at a time, with performance designed in,
   tested end to end and committed before the next. This preparation only reorganizes the
   working mock UI and updates the engineering plan; the next slice shows one real directory.

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
