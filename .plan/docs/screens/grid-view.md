# Screen: Grid view and context menu (Milestone 1)

Reference: `reference/screens/grid-dark-glass@2x.png` (and `-solid`, `grid-latte-*`).
Pins: `reference/screens/grid-anatomy@2x.png`. Each number below matches a pin.
Geometry: `reference/redlines.json` → `grid-dark-glass`. Exact source: `reference/html/grid-dark-glass.html`.

**The data shown is the fixed mock content of the reference** (folder `~/Documents`, 6 folders, 10 files, `shore-at-dusk.jpg` selected and right-clicked, the Open With submenu open, `forest-walk.jpg` hovered).
Milestone 1 renders exactly that from hardcoded data. "Data / backend" says where real data comes from later.

## Sidebar

| # | Element | What it is | How it's used | Data / backend |
|---|---|---|---|---|
| 1 | **Sidebar toggle** | Hides or shows the sidebar. The "Rove" word next to it is just the app name, not a button. | Click, or Ctrl+B | Client-side layout state only. |
| 2 | **Favourites** | Fixed places: Home, Downloads, Documents, Pictures, Music, Videos, Projects, Trash. The user can pin more by dropping a folder here. The highlighted row is the folder shown in the active tab. | Click opens the place in the current tab; middle-click opens a new tab. Dropping files on a row copies or moves them there; dropping on Trash trashes them. | XDG user dirs (xdg-user-dir) plus pinned paths in the state file. Opening sends `list`. |
| 3 | **Tags** | Seven tag colours, taken from the Omarchy theme (red, orange, yellow, green, cyan, blue, magenta). The "…" button renames tags. | Click a dot to show every file with that tag, which is a search for `tag:red`. | `search` with a tag query. Tags are stored as user xattrs. |
| 4 | **Network** | Mounted or saved remotes: rclone (gdrive), SMB, SFTP, FTPS, WebDAV, NFS, Dropbox, Taildrop. A green dot means connected; the grey label says "offline". "+" opens Connect to Server. | Click mounts the remote if needed, then opens it. | Mount workers in the backend; `fsinfo` class `network`. |
| 5 | **Devices** | Every drive, with a capacity bar (accent colour, amber above 85% full) and a free-space caption. Removable drives get an eject button; encrypted ones get Unlock. | Click opens the drive; ⏏ ejects it. | `devs` lists devices; `fsinfo` gives the free bytes. |

## Toolbar

| # | Element | What it is | How it's used | Data / backend |
|---|---|---|---|---|
| 6 | **Back · Forward · Up** | History for this tab. Forward is dimmed when there is nothing to go forward to. Up goes to the parent folder. | Alt+← / Alt+→ / Alt+↑ (exact keys follow the keybinding preset). | Client-side tab history; each step sends `list`. |
| 7 | **Folder title and count** | The current folder's name, in Title type, with its item count beside it. | Click the title to get a menu of the parent folders (a jump list). | `listed` gives the count. |
| 8 | **Search** | You never need to click it: just start typing anywhere in the window and the text lands here, filtering this folder instantly. Tab widens the search to all subfolders. Understands `tag:red`. Typing / or ~ instead edits the path (see the Search journey board). | Type anywhere, or Ctrl+F / click. Esc clears and restores the folder. | `filter` while typing; `search` after Tab. |
| 9 | **View switch** | Four views: List, Columns, Grid (selected here), Two panes. Can be remembered per folder (Settings › View). | Click, or Ctrl+1 to Ctrl+4. | Client-side; the view is saved in the state file. |
| 10 | **Sort** | Shows the current sort key ("Name"). Opens the Sort dropdown (see the List view board): key, order, folders first, group by kind, hidden files. | Click opens the dropdown. | `sort`. |
| 11 | **Settings** | Opens the Settings sheet (see the Settings board). | Click, or Ctrl+, | None. |

## Tabs

| # | Element | What it is | How it's used | Data / backend |
|---|---|---|---|---|
| 12 | **Tabs** | Each tab is one open folder with its own history, like browser tabs. The icon is that place's sidebar icon. Documents is a document; Downloads is a download arrow; "rove" uses the code icon because it is a project folder (~/Projects/rove). Only the active tab shows its × close button, and only when 2+ tabs are open. | Click switches tabs. Middle-click or × closes. Drag to reorder. Drop files on a tab to put them in that folder; holding a dragged file over a tab for 0.6 s switches to it. | Client-side; tabs are saved in the state file. |
| 13 | **New tab** | Opens a new tab on the folder you are in. Right-click it for: New Tab Here, New Tab at Home, Reopen Closed Tab, and the recently closed tabs. The tab bar is always visible, even with one tab, so this button is always one click away. See the Tabs journey board. | Click, or Ctrl+T. Right-click shows more options. Ctrl+Shift+T reopens the last closed tab. | Client-side; the closed-tab history lives in the state file. |

## Content

| # | Element | What it is | How it's used | Data / backend |
|---|---|---|---|---|
| 14 | **Group header** | Section title with its count. In Grid view, folders come first ("Folders") and then files ("Files"). With "Group by kind" on, there are more groups (Photos, Videos, Documents…). | Not interactive. | Grouping comes from the sort settings; counts from `listed`. |
| 15 | **Folder tile** | 124 px wide. The icon comes from the user's icon theme; special folders get their themed icon (Code, Pictures, Locked, Cloud…). The name is one line with an ellipsis. Below it is the item count, or a state ("Locked", "Synced"). | Click selects; double-click or Enter opens; drag moves. | `rows` (names and kinds) plus `dirsize` / child count, both scoped to the viewport. |
| 16 | **Selected tile and check badge** | The selection: soft accent fill, a 1 px accent ring and an accent check badge in the corner. | Click selects one; Ctrl+click toggles; Shift+click selects a range; drag a rectangle to select many. | Client-side selection, sent as `rows` indices with `listing`. |
| 17 | **Thumbnail tile** | Photos and videos show a real thumbnail instead of an icon. Video adds a play mark and its duration. | Same as any tile; Space opens Quick Look. | `thumb` for visible tiles only; `thumbcancel` when they scroll away. |
| 18 | **Hovered tile** | The hover state: 5% white fill. No border. | Mouse over. | None. |

## Footer

| # | Element | What it is | How it's used | Data / backend |
|---|---|---|---|---|
| 19 | **Breadcrumb** | Where you are, as device › Home › Documents, each part with its place icon. The current folder is highlighted. | Click any part to jump there; drop files on a part to move them there. | Built from the path; each jump sends `list`. |
| 20 | **Selection summary** | "1 of 16 selected · 4.1 MB". With nothing selected: "16 items". | Not interactive. | Sizes from the `rows` / `meta` replies. |
| 21 | **Free space** | Free space on the drive this folder is on. | Not interactive. | `fsinfo`. |
| 22 | **Icon size** | Grid tile size slider (Grid view only). | Drag, or Ctrl+= / Ctrl+-. | Client-side. |

## Context menu

| # | Element | What it is | How it's used | Data / backend |
|---|---|---|---|---|
| 23 | **Open** | Opens the file in its default app; a folder opens in this tab. | Enter | Client: xdg mimeapps default (`src/open.rs` logic). |
| 24 | **Open With ›** | A submenu of every app that can open this file type. The default is labelled "Default". "Other application…" opens an app picker. Shown here highlighted in the accent, which is the menu hover state. | Hover or → opens the submenu. | Desktop entries for the MIME type. |
| 25 | **Quick Look** | A full-size preview without opening an app: images, PDFs, text and code, media, archives. | Space | `meta`, `code`, `html`, `thumb`, depending on the file. |
| 26 | **Copy · Cut** | Puts the selection on the clipboard (Rove's clipboard and the system one, as file URIs). Cut tiles show at 45% opacity until pasted. | Ctrl+C / Ctrl+X | `clipSet`; paste runs `transfer`. |
| 27 | **Rename…** | Renames in place: the tile's name turns into a field with the name selected and the extension not (see Components › Grid tiles). | F2 | `rename`, with Undo. |
| 28 | **Compress** | Makes an archive next to the file. The default format is shown on the right (.zip). On an archive this row becomes "Extract". | Click | Archive job via `transfer`, bound with `menuId`. |
| 29 | **Tag dots** | Toggles tags on the selection. A tag that is already on draws a ring. The row is hidden on drives that can't store tags. | Click a dot. | `settags`; state from `tagstates`; hidden when `fsinfo.tagable` is false. |
| 30 | **Send to Shelf** | Adds the file to the Shelf in the Omarchy bar, a holding tray you can drag files out of into any app. | Click | The shelf CLI / plugin. |
| 31 | **Copy Path** | Copies the full path as text. | Ctrl+Shift+C | Client: system clipboard (text). |
| 32 | **Properties** | Opens the Properties panel: size, dates, where it is, permissions, photo and media facts. | Ctrl+I | `menuaction` `properties` / `contents` / `facts`. |
| 33 | **Move to Trash** | The only destructive item: danger colour, always last, after a separator. Trashing shows an Undo toast; nothing asks for confirmation. | Del | `trash`; `undo`. |

## Context menu rules
- The order is fixed: **open** group, then **clipboard and edit** group, then **tags**, then **send / share / info** group, then **Move to Trash last**. Groups are split by separators.
- Rows that don't apply are **removed, not disabled**: Compress becomes Extract on archives; the Tags row is gone when the filesystem can't store tags; Open With has no submenu for folders.
- Multi-selection: the title-bearing items pluralise ("Compress 3 items"); Open With is hidden; Rename becomes "Rename 3 items…" (bulk rename).
- Keyboard: ↑/↓ moves the highlight, → opens a submenu, ← or Esc closes a level, Enter activates, and the menu's mnemonic is the first letter.
- Menus are configurable later (Settings › Menus); build the menu from a data list, not hand-placed elements.
