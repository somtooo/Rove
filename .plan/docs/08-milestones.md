# Milestones: vertical slices

Owner: the lead engineer. Revised with som2 on 2026-11-13.
This replaces the old M0–M9, layer-by-layer sequence. The design remains in force.
Compact toolbar/footer behaviour still needs the designer; do not invent breakpoints.

## How we work

One slice delivers one usable behaviour through every layer it needs:

1. Choose a user action or result from the design and journeys.
2. Work in the component that owns it, including its state and tests.
3. Read Flea's implementation. Bring only the required code and its dependencies into
   Rove's fork, with its upstream revision and licence notices. Do not import unrelated
   features or rewrite working backend logic just to make it fit a folder.
4. Connect the component to that code through a narrow request/event boundary. Extend
   existing infrastructure only as the feature needs it.
5. Test the full behaviour, error paths and performance. Compare the fixed mock scenes
   against their references. Try the real native window.
6. Commit a working slice before starting the next one. Split any slice that cannot be
   reviewed or tested as one coherent behaviour.

There is no "finish all UI", "import the backend", or "optimize later" phase. Reusable
components, menus, protocol support and storage grow out of the features that need them.
A slice need not change Flea if the required backend capability is already present.
Check off a slice only when its acceptance checks pass; the list below is not a claim of
implemented backend functionality.

## Performance is part of every slice

Design the work out before optimizing its implementation:

- Directory size must not determine how many UI elements are built. Render visible rows
  plus a bounded overscan, request held ranges in batches, and version virtualized data so
  an unrelated state change does not scan every entry's key.
- Keep filesystem I/O, decoding and heavy processing off Rune's UI thread. A backend
  event should update the affected state, not trigger a full relist or redraw by default.
- Ask for metadata and thumbnails only when the slice needs them. Cancel obsolete work,
  reject stale replies, reuse cached results and bound caches/queues. No per-entry
  threads, timers or wakeups. Do not keep rendering an idle window.
- Measure the entire path: request → backend work → transfer → UI update → visible frame.
  Flea's speed does not prove Rune's layout, text, rendering or event handling is cheap.
- Record cold and warm first-visible times, settled time, p50/p95 frame times and worst
  frames, memory, UI elements built and requests issued. Use small folders and the same
  reproducible 100k-entry fixture, hardware, build profile and cache conditions each time.
  Separate backend, IPC, UI-build, layout and render costs in traces where possible.
- Aim for the display's frame budget (16.7 ms at 60 Hz); investigate repeatable stalls
  rather than hiding them in averages. Compare against our recorded baseline and a
  same-machine Flea run. Published Flea numbers are context, not a portable pass/fail gate.
- If Rune is the bottleneck, make a minimal reproducer, test a targeted patch in the local
  Rune checkout, and carry a reproducible pinned dependency patch only if it fixes the
  measured problem. Do not upgrade Rune or change the visual design speculatively.

Save the commands, fixture details, build/revision information and before/after results
with the slice under `.plan/`. If the specified visual design itself forces excessive
work, bring the evidence to som2 for the designer; do not silently simplify it.

## Foundation already working

- Native mock GUI on pinned Rune `d6a0b3cc`: window, sidebar, toolbar, tabs, grouped file
  grid and footer, with theme tokens, Inter and JetBrains Mono.
- Tile hover/selection, Esc clearing, sidebar toggle, equal hidden-sidebar insets, bounded
  grid scrolling and independently scrolling sidebar with pinned brand.
- Four original material scenes, plus separate 90%-panel Glass review variants. The
  original Glass tokens are unchanged.
- `src/components/` organized by design component. Parts and behaviour tests live with
  their owner. `src/app/` owns startup/shared state; `src/fixtures/` owns mock data/scenes.
- Eleven tests and four reference checks pass. The reference-size checks alone do not
  prove compact toolbar/footer correctness or real-directory performance.

**Current delivery:** this component reorganization and slice plan. No backend is wired.
**Next implementation:** S1, after this preparation is committed.

## S1 — Show one real directory in the grid

**User result:** launch Rove with an explicit local directory and see its actual folders
and files, not the Documents fixture. Read-only; no navigation or file actions yet.

- **UI:** `components/file_grid/` owns the entry model, grouped rows and bounded viewport.
  Feed real name/type data to tiles. Replace relevant title/count/footer fixture values
  with the active directory's values. Do not invent metadata or show fixture devices as
  real measurements. Preserve fixed mock scenes as a separate deterministic data source.
- **Flea → Rove:** bring the listing/session/protocol code and the dependency closure
  needed to build and run it into `crates/rove-backend/`. Record the upstream revision
  and MIT notices. Inspect process startup and helper dependencies; do not strip them
  blindly or call process-owning code in the UI process. No installed Flea dependency.
- **Boundary:** child-process isolation first (`01-architecture.md`). Implement only the
  required list/window/reply/shutdown path. Batch updates; use listing identities to
  reject obsolete results. Virtualize rows of tiles and hold only a bounded UI range.
  Keep the backend's directory index where needed; do not duplicate all rows into UI
  elements. Use existing generic/theme icons, not new checked-in reference artwork.
- **Tests:** a temporary directory with known folders/files, an empty directory, a failed
  listing, stale replies, range changes and clean shutdown. Confirm the right names and
  grouping reach the UI. All entries must remain reachable by scrolling.
- **Performance:** instrument first-visible and settled time, UI work/range requests,
  frame time and memory on small and 100k-entry directories. Establish the first real
  end-to-end baseline; show that mounted UI work stays bounded by the viewport.
- **Done:** a native read-only grid shows the chosen directory, errors do not freeze the
  UI, the fork builds from this repository alone, tests/reference checks pass and the
  measured baseline is recorded. One working feature commit.

## S2 — Enter a folder and return

**User result:** open a folder, go Up, Back and Forward, and see the correct location.

- Extend grid activation, toolbar navigation and footer breadcrumbs together.
- Bring the required Flea path/history helpers and request guards into the fork.
  Cancel the old location's work and prevent late replies from replacing the new one.
- Test parent/root handling, inaccessible targets, rapid navigation and history.
  Measure navigation-to-first-visible and requests per location; avoid eager tree scans.
- **Done:** navigation and displayed path/count agree end to end; mock scenes still pass.

## S3 — Keep the displayed directory current

**User result:** external changes and explicit Refresh update the current listing.

- Connect the needed Flea watcher/changed/relist path to the active location only.
  Preserve selection and scroll position by stable identity where possible.
- Test create/remove/rename, watcher bursts, leaving a watched directory and stale replies.
  Measure idle wakeups, relist frequency and burst handling; batch/coalesce changes.
- **Done:** the grid becomes correct without reopening the folder, with no idle repaint loop.

## S4 — Open a sidebar favourite

**User result:** clicking Home, Documents or another configured place navigates there.

- Resolve actual XDG user directories using the audited Flea place helpers. Sidebar
  selection, grid location, toolbar and breadcrumbs must agree. No network/device UI
  claims until those separate flows work.
- Test missing/overridden XDG directories and repeated switching. Cache place discovery
  instead of resolving it every frame.
- **Done:** a sidebar click uses the same working navigation path as S2.

## S5 — Select real entries with pointer and keyboard

**User result:** select, extend, clear and move the selection over real directory entries.

- Own grid cursor/selection beside `file_grid`; reflect it in the footer. Bring only the
  keymap and selection helpers that match `11-keybindings.md` and the relevant journeys.
- Test 2D movement, range/toggle selection, focus, wrapping after resize, stale/deleted
  entries and scrolling to an unmounted selected row. Do not scan all entries on every key.
- **Done:** the selected real entries and footer summary agree across navigation/refresh.

## S6 — Switch between grid and list

**User result:** switch presentation without changing directory or losing selection.

- Add the designed list component, backed by the same listing/range model as the grid;
  use virtual 32 px rows. Do not launch another backend or duplicate the full directory.
- Test both directions of the switch, range requests and selection/focus continuity.
  Capture the list references and measure UI/frame/memory costs on the large fixture.
- **Done:** both presentations browse the same real entries with bounded mounted work.

## Following slices: one behaviour per commit

Choose these in order of user value and dependencies, keeping the same acceptance gates.
Read the relevant design and journeys before starting each one; split further when needed.

- **Change sorting:** sort dropdown → backend ordering → stable/reloaded listing.
  Build the reusable in-window menu here, including its designed keyboard behaviour,
  bounds, submenu timing/safe triangle where needed. No standalone menus milestone.
- **Show file icons:** backend icon name → freedesktop theme lookup → cached tile/list art.
  Audit Flea's app-icon resolver before adapting it; keep GPL reference images local.
- **Show real thumbnails:** visible entry → cancellable Flea thumbnail request → decoded
  image. Prove off-screen work is cancelled and cache/decoding memory stays bounded.
- **Open a file:** activation → audited Flea launcher → system application, with errors.
- **Open With:** menu/submenu → application discovery → chosen launcher; reuse and test
  the existing menu rather than building a separate UI-only screen.
- **Rename:** inline editor → guarded backend rename → refreshed identity/selection.
- **Trash and Undo:** guarded trash operation → listing update/toast → designed undo.
- **Copy, cut and paste:** selection → Flea file clipboard → guarded transfer/progress.
  Reuse the backend's Wayland clipboard support; Rune text clipboard is not file cut.
- **Persist view preferences:** one working setting at a time → component behaviour →
  Rove's own store at `$XDG_STATE_HOME/rove/ui.json`. Add the corresponding settings
  controls with the feature, not an entire nonfunctional settings sheet first.
- **Follow the live theme:** theme change → watcher → token resolution → affected
  components, with no unrelated relisting or per-entry updates.
- **Drag files:** the designed internal/external copy/move flow → transfer completion.
  Rune's Wayland drop-in and Linux drag-out gaps require a measured framework spike
  within this slice; verify on Hyprland, not only headlessly.
- **Inspect properties, compress/extract, tags:** separate end-to-end slices, not a single
  "Actions" batch. Get any missing Properties design through som2 first.

Columns, two panes, Quick Look, search results, Trash browsing, remote/device flows,
the file-chooser portal and remaining settings pages also follow this rule. Missing
visual specifications and compact layouts go to the designer before implementation.

## Optional backend/library extraction

This is not a prerequisite or a promised final milestone. When a concrete slice or a
profile justifies it, isolate process-wide stdio, allocator, signals/limits, helpers and
shutdown in Rove's fork. Compare child-process and in-process implementations on the
same workload (latency, memory and crash behaviour). Switch only on evidence and keep
a child-process route where it is useful. Never patch the reference Flea checkout.

## Commit and publication gate

- Run `cargo test -p rove-ui`, `cargo clippy -p rove-ui --all-targets -- -D warnings`,
  `cargo build -p rove-ui`, then `python3 -I tools/verify.py`.
- Include native/end-to-end tests and recorded performance evidence for the slice,
  not just mock screenshot matches. Preparation-only commits do not claim such evidence.
- Preserve unrelated owner edits. Review the staged diff and the public branch history.
- Keep WhiteSur icons, wallpaper, derived screenshots and other excluded third-party
  art out of commits, including renamed copies. Publish only the audited `main` branch;
  never `--mirror`, all refs, or the local saved history containing the original art.
