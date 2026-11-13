# Keybindings

**Rules:**
- Bare printable keys belong to **type-to-search** (Default, Mac and Windows presets).
- Commands use modifiers, function keys, or navigation keys.
- Rove never binds `Super` (Hyprland's).
- Presets change keys, never actions.
- Every menu item shows its key.
- **F1** / **Ctrl+/** opens the shortcut sheet.
- Flea's `keys.toml` already carries the four-preset table. Update it to this map rather than writing a new one.

## Default preset
| Area | Key | Action |
|---|---|---|
| **Typing** | any letter/digit | filter this folder (type-to-search) |
| | `/` or `~` (as the first character) | path entry |
| | Ctrl+F | focus search |
| | Ctrl+L | path entry |
| | Tab / Shift+Tab (while searching) | search all subfolders / this folder only |
| **Navigate** | Enter | open |
| | Ctrl+Enter | open in a new tab |
| | Shift+Enter | Open With menu |
| | Alt+← / Alt+→ | back / forward |
| | Backspace (no search text) | back |
| | Alt+↑ | up to the parent |
| | Alt+Home | Home |
| | F5 / Ctrl+R | reload |
| **Tabs** | Ctrl+T | new tab here |
| | Ctrl+W | close tab (the last one closes the window) |
| | Ctrl+Shift+T | reopen closed tab |
| | Ctrl+Tab / Ctrl+Shift+Tab | next / previous tab |
| | Ctrl+PgDn / Ctrl+PgUp | next / previous tab |
| | Alt+1…9 | tab 1…9 (9 = last) |
| **View** | Ctrl+1 / 2 / 3 / 4 | List / Columns / Grid / Two panes |
| | Ctrl+= / Ctrl+- / Ctrl+0 | bigger / smaller / reset icons |
| | Ctrl+H | show hidden files |
| | Ctrl+B | sidebar |
| | Ctrl+Shift+I | inspector |
| **Select** | arrows | move the cursor |
| | Shift+arrows | extend |
| | Ctrl+arrows | move without selecting |
| | Ctrl+Space | toggle the item at the cursor |
| | Ctrl+A | select all |
| | Esc | clear (see the Esc ladder) |
| | Home / End, PgUp / PgDn | jump |
| **Files** | Space | Quick Look |
| | F2 | rename (Tab = save and go to the next) |
| | Ctrl+C / Ctrl+X / Ctrl+V | copy / cut / paste |
| | Ctrl+Shift+V | paste as link |
| | Ctrl+D | duplicate |
| | Ctrl+Shift+N | new folder |
| | Ctrl+Alt+N | new file |
| | Del | move to Trash |
| | Shift+Del | delete permanently (confirms) |
| | Ctrl+Z / Ctrl+Shift+Z | undo / redo |
| | Ctrl+Shift+C | copy path |
| | Ctrl+I | properties |
| | Ctrl+Shift+1…7 | toggle tag 1…7 |
| | Ctrl+Shift+0 | clear tags |
| | F4 | open terminal here |
| | Menu key / Shift+F10 | context menu |
| **App** | Ctrl+, | settings |
| | F1 / Ctrl+/ | keyboard shortcuts |
| | Ctrl+N | new window |
| | Ctrl+Q | quit |

## Inside menus, dialogs and sheets
- **Menus:** ↑/↓ move, → opens a submenu, ← or Esc closes a level, Enter activates, Home/End jump, and typing a letter jumps to the item starting with it.
- **Dialogs and sheets:** Enter = the safe default button, Esc = Cancel, Tab cycles controls.
- **Rename field:** Enter saves, Esc cancels, Tab saves and moves to the next item, Shift+Tab to the previous.

## Vim preset (differences)
Letters are commands here, so **type-to-search is off**. `/` searches (and `n` / `N` jump between matches). Path entry is Ctrl+L or `:` then a path.

| Key | Action |
|---|---|
| h / l | up / open |
| j / k | down / up |
| gg / G | first / last |
| yy / dd / p | copy / move to Trash / paste |
| x | cut |
| u / Ctrl+R | undo / redo |
| v | start a range selection |
| r | rename |
| o | new folder |
| t1…t7 | toggle tag |
| gt / gT | next / previous tab |
| Space | Quick Look |

## Mac preset
- Cmd replaces Ctrl throughout.
- Cmd+↑ goes up, Cmd+↓ opens, Cmd+Backspace moves to Trash.
- Cmd+Shift+. toggles hidden files, Cmd+Y is Quick Look (Space still works).

## Windows preset
- Backspace goes **up** (Explorer), Alt+Enter is properties, F3 / Ctrl+E focuses search.
- Ctrl+Shift+N makes a new folder (same), Shift+F10 opens the context menu.
- Type-to-search stays on.

## Reserved: never bind
- `Super+anything` (Hyprland and Omarchy).
- Ctrl+Alt+T (common system terminal key).
- Print and media keys.
