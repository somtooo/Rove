# Motion and input

## Motion (Rune `rune::tok::motion`)
| What | Motion |
|---|---|
| Hover fill in | instant; fade out 120 ms |
| Press (buttons, tiles) | scale .97, 90 ms; release on SNAPPY |
| Menus, dropdowns, tooltips | in: opacity 0→1 + translateY −4→0 on SNAPPY (0.25 s, no bounce); out: opacity, 120 ms |
| Submenu | opens after 120 ms of hover on the parent row; stays while the pointer travels towards it (safe triangle) |
| Sheet (Settings, Properties) | scrim fade 200 ms; sheet scale .98→1 + opacity on DEFAULT (0.3 s) |
| Selection change | colours retarget on SNAPPY; the check badge scales .6→1 |
| View switch | cross-fade 150 ms; no layout spring |
| Theme change (Omarchy switch) | colours only, CALM (0.4 s) in Oklab, text leading its background (Rune does this) |
| Toasts | in from below 12 px on DEFAULT; out by fade |
| Reduced motion | no travel or scale; opacity only, 0.15 s |

Nothing animates on the first frame.

## Focus
- Keyboard focus: a 2 px accent ring, offset 2, following the element's radius. Shown only for keyboard navigation (`:focus-visible` semantics).
- Grid and list have a keyboard cursor distinct from selection: a 1.5 px inset accentLine ring on the cursor item without fill. Selection keeps its fill.
- Tab order: sidebar, toolbar, tabs, content, footer. Arrow keys move inside each.

## Pointer
- **Click** selects. **Ctrl+click** toggles. **Shift+click** selects a range. **Drag on empty space** draws a rubber band (fill accent @ .12, border 1 px accentLine, radius 4).
- **Double-click** or **Enter** opens. **Middle-click** opens a folder in a new tab.
- **Right-click** shows the context menu for the item under the pointer, selecting it first if it wasn't selected.
- **Drag and drop:** same device moves, other device copies; Ctrl copies, Shift moves, Ctrl+Shift links. Drop targets show the accent ring.

## Keyboard
The complete map, with the Vim, Mac and Windows presets, is in **`11-keybindings.md`**. The rules:
- Bare letters start type-to-search.
- `/` or `~` starts path entry.
- Commands use modifiers or function keys.
- Rove never binds Super.
- Flea's `keys.toml` holds the preset table to update.
