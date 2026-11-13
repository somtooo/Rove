# Components

Visual reference: `reference/screens/components@2x.png`; exact CSS: `reference/html/components.html`.
Every component reads only tokens, so Mode × Material × accent come for free.

## Buttons
| Variant | Fill | Text | Use |
|---|---|---|---|
| primary | accent | ink, 600 | the one main action (Open, Save) |
| secondary | fill2 | t1, 500 | other actions |
| ghost | none | t2, 500 | Cancel, toolbar text buttons |
| danger | danger @ .18 | danger | Empty Trash in a strip; in a confirm dialog the button is a danger fill with ink text |

- **Sizes:**
  - sm: height 26, padding 0 9, radius 7, 12 px
  - md: height 32, padding 0 12, radius 9, 13 px
  - lg: height 40, padding 0 16, radius 11, 14 px
- Icon 16 (sm: 14), gap 7.
- **States:**
  - hover: brightness +10% (secondary/ghost: add `hover`)
  - pressed: scale .97 and brightness −8%, 90 ms
  - focus: 2 px accent ring, offset 2 (keyboard focus only)
  - disabled: opacity .4

## Icon button
- 30×30, radius 8, icon 16–18 in t2.
- hover: `hover` fill.
- selected (toggle): `fill3` + control_on shadow, icon t1.
- disabled: icon t4.
- Always has an accessible label and a tooltip with its shortcut.

## Segmented control and view switch
- Track: fill, radius 9 (view switch 10), padding 2, gap 2.
- Segments: height 26, padding 0 11, radius 7, 12.5/500 t2. Icon-only segments are 30 wide.
- Selected: `fill3` + control_on shadow, t1.
- Stepper: the same track with − / value (13/600, 26 wide) / +.

## Switch, checkbox, radio
- **Switch:** 34×20, radius 10. Off: white @ .16 (Light: black @ .12). On: accent. Knob 16 white with shadow `0 1 3 rgba(0,0,0,.4)`, travelling 14 px on SNAPPY.
- **Checkbox:** 18×18, radius 5. Off: inset 1.5 px white @ .28. On: accent fill, check 12 in ink at stroke 3. Mixed: a minus icon.
- **Radio:** 18 circle. Off: inset 1.5 px white @ .28. On: inset 5 px accent.

## Fields
- **Text field:** height 34, padding 0 10, radius 9, background white @ .05, inset 1 px line2, 13.5 t1, placeholder t4.
  - focus: inset 1 px accent + outer 3 px accent @ .25
  - error: the same in danger, with a 12 px danger message below
- **Search:** see `04-layout.md`; Esc clears.
- **Select:** height 30, padding 0 8 0 11, radius 8, fill2, 13/500, chevron 14 t3. Open: inset 1 px accentLine, plus a dropdown menu (a menu with a check slot).
- **Inline rename** (grid tile):
  - field 122×24, margin 0 −7, radius 6, 12.5/500, background panel, focus ring
  - the name **without its extension** is pre-selected (accent @ .45 highlight)
  - a caption below: "Enter to save"
  - Esc cancels

## Menu
See `04-layout.md › Menus`. One component serves context menus, submenus, dropdowns and selects. Data-driven items:
`Item { icon?, label, trailing: Keys([..]) | Text | Submenu | None, kind: Normal|Destructive|Check(bool), enabled }`,
separators, section labels and the tag row.

## Tooltip
- Height 28, padding 0 8 0 10, radius 8, popover background, border line2, shadow `0 8 24 rgba(0,0,0,.4)`, 12.5 px.
- Keycaps on the right.
- Appears after 500 ms of rest; hides on move-out or press.

## Toast (bottom-right stack, 16 px from the panel edges, 8 apart, width 420)
- **Container:** padding 12 12 12 14, radius 12, popover background, border line2, shadow `0 16 40 rgba(0,0,0,.45)`.
- **Leading badge:** 36×36, radius 10, tinted @ .18, icon 18.
  - progress toasts use the accent
  - success uses success
  - errors use danger
- **Progress toast:**
  - title 13.5/500 with a right-aligned % (12 t3)
  - a 4 px bar
  - caption "1.1 GB of 2.6 GB · about 12 seconds left"
  - × cancels
- **Undo toast:** a message plus a small secondary "Undo" button with a `Z` keycap. Auto-dismisses after 6 s; hover pauses it.
- **Error toast:** title plus a reason caption, and Retry.

## Confirmation dialog
- 400 wide, radius 16, popover background, border line2, shadow `0 30 80 rgba(0,0,0,.55)`, padding 22, gap 16.
- 40×40 danger @ .18 badge, title 16/600 −1%, body 13.5/20 t2.
- Buttons right-aligned: Cancel (ghost, with an Esc keycap) and the verb (danger fill, ink text).
- Only for irreversible actions. The button says the verb ("Empty Trash"), never "OK".

## Empty folder
- Centred in the content area.
- The icon-theme folder icon 72 at opacity .55.
- "This folder is empty" 15/600.
- Hint 13 t3: "Drop files here, or press Ctrl Shift N for a new folder."

## Sidebar rows, devices, tabs, crumbs, tiles, list rows
See `04-layout.md`. States are shown on the components board: rest, hover, selected, drop target, cut, renaming.
