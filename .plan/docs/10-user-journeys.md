# User journeys: the first feature set

How Rove *feels* to use: every core task, step by step, with what it costs, what the
user sees, and the edge cases. Each journey ends with a review. I walked through it as a user,
asked whether it felt good, and changed the design where it didn't. Those changes are collected in
the **Iteration log** at the end and are already reflected in every other doc and reference.

Storyboards: `reference/screens/tabs-journey@2x.png` and `search-journey@2x.png` (also on the canvas).
Keys: the complete map is in `11-keybindings.md`. This doc uses the **Default** preset.

## Principles (the yardstick for every journey)
1. **Click budget.** Frequent tasks take **≤ 2 actions**, occasional ones **≤ 3**. Anything over that is a design bug. An "action" is a click, a key chord, or one typed word.
2. **Just type.** Letters search. `/` or `~` types a path. Nothing needs to be "opened" first.
3. **Nothing modal unless it can't be undone.** Every reversible action happens immediately and offers **Undo** (toast + Ctrl+Z). Dialogs exist only for Empty Trash, Delete Permanently, and choices the app can't guess (name conflicts).
4. **Keyboard and mouse are equals.** Every action has both, and menus always show the keys.
5. **Feedback within one frame.** The hover, selection or filter result shows on the next frame. Slow work (copy, search, sizes) streams in and never blocks.
6. **Respect Hyprland.** Rove never binds `Super`; that belongs to the compositor. Omarchy's own keys stay untouched.
7. **The Esc ladder.** Each Esc undoes exactly one layer, in this order:
   1. close a menu, popover or tooltip
   2. cancel a rename or a drag
   3. clear the search
   4. clear the selection
   5. *(opt-in setting)* go up a folder

---

## J1: First launch and getting your bearings
**Goal:** open Rove and understand where you are.
1. Launch (Super+Shift+F on Omarchy, or `rove` in a terminal).
2. Rove opens **Home** in one tab, in Grid view, with the sidebar showing Favourites, Tags, Network and Devices.
3. The title says "Home" with its item count. The footer breadcrumb says `som2omarchy › Home`. The `+` sits next to the tab.

- **Cost:** 0.
- **Later launches:** restore the last session (tabs, folders, scroll, view). The Settings › View › "Rove opens in" option offers Last session, Home, or a chosen folder.
- **Edge cases:**
  - The last folder no longer exists: open its nearest existing parent, with a toast "‹folder› was moved or deleted".
  - A remote is offline: the tab opens with an inline "Can't reach nas · Retry" state, not an error dialog.

**Review: good.** Zero-step start, no welcome screen, no tour.

## J2: Go to a folder
| How | Steps | Cost |
|---|---|---|
| A sidebar place | click it | 1 |
| A folder you can see | double-click (or select + Enter) | 1 |
| Somewhere up the path | click a breadcrumb | 1 |
| A known path | type `/` or `~`, type, Tab-complete, Enter | 1 word |
| Back / forward / up | Alt+← / Alt+→ / Alt+↑ (or toolbar buttons; Backspace = Back) | 1 |
| A folder by name in this folder | type its first letters, Enter | 1 word |

- **Feedback:**
  - the title, tab label and icon, breadcrumb and sidebar highlight all update together
  - the first screen of files paints immediately, with the rest streaming in (Flea's `list` + `rows`)
  - scroll position is remembered per folder in history, so Back lands where you were
- **Edge cases:**
  - Permission denied: an inline empty state, "You don't have permission to open ‹folder›", with "Open as administrator" only if polkit is available.
  - A folder that is slow to list (network): a skeleton grid after 150 ms. Never a spinner over old content.

**Review:** all paths are ≤ 1 action. I changed **Backspace**. It was Up; it's now **Back**, matching browsers and Thunar. Alt+↑ is Up. (Iteration #6.)

## J3: Tabs
Full storyboard: `tabs-journey@2x.png`. Summary:
- The tab bar is **always visible**, so `+` is always one click away.
- A plain click on a sidebar place, crumb or folder **navigates the active tab**. A middle-click or Ctrl+click **opens it in a new tab**.
- `+` / Ctrl+T opens a new tab on the current folder. Right-click `+` offers New Tab at Home, Reopen Closed Tab, and the recently closed tabs.
- Close with × (only when 2+ tabs are open), a middle-click on the tab, or Ctrl+W.
- **Cost:** launch → three folders in three tabs = 4 clicks (or 3 using middle-clicks).
- **Edge cases:**
  - Two tabs with the same folder name: the parent is appended ("src · rove").
  - Dragging a file over a tab for 0.6 s switches to it.
  - Ctrl+W on the last tab closes the window (the session is saved first).

**Review:** the original spec hid the tab bar with one tab, which hid `+` and made "new tab" invisible to new users. **Changed: always visible.** (Iteration #1.)

## J4: Find something: just type
Full storyboard: `search-journey@2x.png`.
1. Type `inv` anywhere. The current folder **filters instantly**: matches are highlighted, the first is selected, and the count shows ("2 of 16").
2. Enter opens the selected match; arrows move; Space previews; Ctrl+Enter opens in a new tab.
3. Nothing here? Press **Tab**: the same text searches **all subfolders**, ranked by match quality then recency, each result showing where it lives.
4. Esc clears and restores your previous selection and scroll.

- **Cost:** 1 word to a result in this folder; +1 key for everywhere.
- **Syntax:** `tag:red`, `kind:image`, `>10mb`, `modified:today`. Suggestions appear after `:`.
- **Edge cases:**
  - No matches: "No matches in Documents · Tab to search subfolders".
  - Huge trees: results stream in, and the count says "120+ so far".
  - Typing inside rename or a settings field never triggers search.

**Review:** this is the one Linux users expect (GTK/Nautilus type-ahead), and it beats a search box. Walking through it found two conflicts:
- Single-letter shortcuts (`T` then 1–7 for tags, `?` for help) would steal letters from search. Fixed: they now use modifier keys. (Iterations #2, #3.)
- `/` was "focus search", but on Linux `/` means "type a path". Fixed: `/` and `~` start path entry, and Ctrl+F is the search hint on the field. (Iteration #4.)

## J5: Change how the folder looks
| Want | Do | Cost |
|---|---|---|
| List / Columns / Grid / Two panes | Ctrl+1 / 2 / 3 / 4, or the toolbar view switch | 1 |
| Bigger or smaller icons | Ctrl+= / Ctrl+- (Ctrl+0 resets), or the footer slider | 1 |
| Sort | click the Sort button → pick a key; click a list column header; the same key again flips the order | 2 (1 in List) |
| Show hidden files | Ctrl+H | 1 |
| Show / hide the sidebar | Ctrl+B | 1 |
| Show / hide the inspector | Ctrl+Shift+I | 1 |

- **Remembering:** the view, sort and icon size apply to *this folder* if "Remember each folder's view/sort" is on, otherwise everywhere. The view switch never asks.

**Review: good.** I considered a single "cycle view" key and rejected it: direct Ctrl+1–4 is faster and matches Nautilus and Dolphin.

## J6: Select files
- **Click** selects one; **Ctrl+click** toggles; **Shift+click** selects a range; **Ctrl+A** selects all; **Esc** clears.
- **Rubber band:** drag from any empty space, including the gaps between tiles and the area below the last row. Rove keeps at least 24 px of droppable empty space below the last row for this.
- **Keyboard:**
  - arrows move the cursor
  - Shift+arrows extend
  - Ctrl+arrows move the cursor without selecting, and Ctrl+Space toggles the item under it
  - Home/End and PgUp/PgDn work as expected
- **Feedback:** the footer always shows "N of M selected · size".

**Review:** a full grid can leave no empty space to start a rubber band. **Fixed with the 24 px minimum tail.** (Iteration #7.)

## J7: Peek at a file (Quick Look)
1. Select a file and press **Space**. A large preview sheet opens over the window (image, PDF with pages, text and code highlighted, audio and video playable, archive contents).
2. ←/→ (or ↑/↓ in List) moves to the next file **without closing**.
3. Space or Esc closes it. Enter opens it in its app.

- **Cost:** 1 key.
- **Edge cases:**
  - An unsupported type shows the big icon, kind, size and dates, plus "Open with ‹default app›".

**Review: good.** It matches what macOS users expect, and Flea's Quick Look already covers the formats.

## J8: Open a file in its app, or another app
| Want | Do | Cost |
|---|---|---|
| Default app | double-click / Enter | 1 |
| Another app, once | right-click → Open With → app | 3 |
| Change the default | right-click → Open With → Other Application… → pick → tick **"Always use for .jpg files"** | 4 (rare) |
| Folder in a terminal | right-click → Open in Terminal, or **F4** | 1–2 |

**Review:** "another app" costs 3, inside the budget for an occasional task. I added **Shift+Enter → Open With menu at the selection**, making it 2 for keyboard users. (Iteration #8.)

## J9: Copy or move files
**By dragging** (1 action):
- Drag onto a folder, a sidebar place, a tab (hover 0.6 s switches to it) or a breadcrumb.
- Same drive = move; other drive = copy. Hold Ctrl to copy, Shift to move, Ctrl+Shift to link.
- The drag badge shows the count and the verb ("Copy 3").

**By clipboard:** Ctrl+C or Ctrl+X → go there → Ctrl+V (3 actions + navigation). Cut items show at 45% opacity until pasted.

**After the drop or paste:**
- the new items are **selected and scrolled into view**
- quick jobs (under 1 s) finish silently; longer ones show a **progress toast** (bytes, time left, ×), and several jobs stack in one toast
- every copy and move can be undone (Ctrl+Z)

**Name conflicts** (the one dialog Rove can't avoid):
- One sheet for **all** conflicts in the job, not one per file.
- It lists them with newer/older and sizes, and offers **Replace All · Keep Both · Skip All**, plus per-row overrides.
- Enter = Keep Both, the safe default.

**Review:** the first draft asked per file, which would be 14 dialogs for 14 conflicts. **Changed to one batched sheet.** Also added "select the pasted items", because without it users can't find what they just pasted. (Iterations #9, #10.)

## J10: Rename, new folder, duplicate
- **Rename:** F2 (or right-click → Rename, or a slow second click on the name).
  - The name is selected **without its extension**.
  - Enter saves, Esc cancels.
  - **Tab saves and moves to the next file's name**, for renaming a series.
- **Rename several:** select them → F2 → the Bulk Rename sheet (find/replace, numbering, case) with a live preview of old → new.
- **New folder:** Ctrl+Shift+N creates "Untitled folder" **already in rename mode**. Type the name, Enter. No dialog. (1 chord + 1 word.)
- **New file:** Ctrl+Alt+N, the same way.
- **Duplicate:** Ctrl+D creates "name copy.ext", selected.
- Every one of these can be undone.

**Review: good.** New folder is 2 actions from anywhere, and Tab-to-next makes series renames fast.

## J11: Trash, undo, delete for good
1. **Del** (or right-click → Move to Trash, or drag to Trash in the sidebar). The items disappear and a toast says "Moved 3 items to Trash · **Undo**".
2. Undo: click Undo or Ctrl+Z. The items come back, selected.
3. **Delete permanently:** Shift+Del brings up a confirm dialog "Delete 3 items permanently? This can't be undone." with the **Delete** button (danger) and Cancel. Esc = Cancel.
4. **Empty Trash:** open Trash → the "Empty Trash" button in the strip → confirm.

- **Restore from Trash:** in Trash, select → **Restore** (button, or right-click). Items go back to where they came from.

**Review: good.** No confirmation for trashing, because it's undoable. Confirmation only where it's final.

## J12: Tag files and find them later
1. Select → right-click → click a tag dot (2 clicks), or **Ctrl+Shift+1–7** (1 chord). Click the dot again (or the same chord) to remove it.
2. Find: click the tag dot in the sidebar (1 click) to see everything with that tag, or type `tag:red`.

- **Edge cases:** a filesystem that can't store tags (some network shares, FAT): the tag row is absent from the menu, and a chord shows the toast "Tags can't be saved on this drive".

**Review:** Flea's `T` then 1–7 was fast but kills type-to-search. **Changed to Ctrl+Shift+1–7.** (Iteration #2.)

## J13: USB drive in, files out, eject
1. Plug it in. The drive appears under Devices with a capacity bar, and a toast says "SanDisk Ultra connected · **Open**".
2. Open: click Open in the toast or the sidebar row (1 click). An encrypted drive shows **Unlock** first; the password goes in an inline field on the row, not a separate window.
3. Drag files onto its sidebar row to copy (1 drag).
4. Eject: ⏏ on the row (1 click). Rove waits for pending writes, then shows "Safe to remove SanDisk Ultra".

- **Busy drive:** the toast says "SanDisk Ultra is in use by **mpv**" with **Eject anyway** / Cancel. *(The backend needs to report the holding process; to add.)*
- **Nearly full** (above 85%): the bar turns amber. A copy that won't fit is refused before it starts: "Needs 2.1 GB, 1.4 GB free".

**Review: good.** The whole USB round trip is 3 actions. The eject feedback is explicit, so users trust pulling the stick.

## J14: Change a setting
1. **Ctrl+,** (or the toolbar sliders button). The Settings sheet opens on the last page used.
2. Type to search settings ("hidden" finds Show hidden files).
3. Changes apply **instantly**. There's no Save. Esc closes.

- Dependent settings (Hidden files last) are dimmed, with the reason written under them.

**Review: good.** The search field earns its place because there are 40+ settings across 8 pages.

## J15: Change the Omarchy theme
- Switch theme in Omarchy. Rove recolours **only** the accent-driven parts and the tags, and switches Light/Dark if the theme's mode changed, over 0.4 s.
- **Cost:** 0.

**Review: good.** This is the principle the owner cares about most: the theme accents Rove, it doesn't repaint it.

---

## Click-budget scorecard
| Task | Mouse | Keyboard | Budget | ✓ |
|---|---|---|---|---|
| Open a favourite place | 1 | 1 word (`~/Dow…`) | 2 | ✓ |
| Find a file in this folder | — | 1 word | 2 | ✓ |
| Find a file anywhere below | — | 1 word + Tab | 2 | ✓ |
| Two folders in tabs from launch | 3 | 3 | 3 | ✓ |
| Switch view | 1 | 1 | 2 | ✓ |
| Preview | — | 1 | 2 | ✓ |
| Open with another app | 3 | 2 | 3 | ✓ |
| Copy / move | 1 drag | 3 + navigation | 3 | ✓ |
| Rename | 2 | 1 + word | 2 | ✓ |
| New folder | 2 | 1 + word | 2 | ✓ |
| Trash + undo | 1 + 1 | 1 + 1 | 2 | ✓ |
| Tag | 2 | 1 | 2 | ✓ |
| Eject | 1 | — | 1 | ✓ |
| Change default app | 4 | 3 | 4 (rare) | ✓ |

## Iteration log (what changed after walking the journeys)
| # | Was | Now | Why |
|---|---|---|---|
| 1 | Tab bar hidden with one tab | Always visible | `+` must be findable from the first second (owner feedback) |
| 2 | Tags: `T` then 1–7 | **Ctrl+Shift+1–7** | Bare letters must start search |
| 3 | `?` opens the shortcut sheet | **F1** or **Ctrl+/** | `?` is printable, so it must start search |
| 4 | `/` focuses search; search field shows a `/` keycap | `/` or `~` starts **path entry**; search field shows **Ctrl F** | Linux convention: slash means path |
| 5 | Esc behaviour undefined | The **Esc ladder** (Principles #7); "Esc goes up a folder" is opt-in | Predictable Esc |
| 6 | Backspace = Up | Backspace = **Back**; Alt+↑ = Up | Matches browsers and Thunar |
| 7 | No space guaranteed for a rubber band | 24 px empty tail below the last row | Full grids had no place to start a drag |
| 8 | Open With only via the menu (3) | + **Shift+Enter** opens the Open With menu (2) | Keyboard parity |
| 9 | Conflict dialog per file | **One batched conflict sheet** | 14 conflicts must not mean 14 dialogs |
| 10 | Pasted items not selected | Pasted and dropped items **selected and scrolled into view** | Users lose what they just pasted |
| 11 | No zoom reset | Ctrl+0 resets icon size | Standard |
| 12 | Single-tab × closes the window | No × on a single tab; Ctrl+W still closes | Avoid accidental window close |
