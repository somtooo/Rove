# Layout: geometry of the main window

All values are logical px. Coordinates are relative to the **window's outer top-left**. In
the 1440×920 references the window sits at (40, 40) and is 1360×840. Every element listed
here is also in `reference/redlines.json`, with computed style.

The layout is flexbox all the way down, which maps 1:1 onto Rune's `row()` / `col()` with
Taffy. Widths marked *flex* fill the remaining space.

## Window
- **Window:** 1360×840 in the reference. Minimum size 720×480.
- **Corner radius:** 16. On Hyprland the compositor rounds the window; match it with `decoration:rounding`.
- **Border:** 1 px window-border colour.
- **Shadow:** the window shadow token. On Hyprland the compositor draws it; skip it in-app there.
- **Background:** `base`. In Glass it is translucent and the compositor blurs behind it.
- **Children:** `row()` = sidebar (fixed 244) + panel (flex).

## Sidebar (x 1, width 244)
- Padding 12 top / 10 right / 12 bottom / 12 left. Vertical stack, no gap; spacing comes from each item.
- **Brand row:** height 36, padding 0 2 0 10.
  - "Rove": 15/600, letter-spacing −1%.
  - Sidebar toggle on the right: icon button 30×30, radius 8, icon 16.
- **Section label** ("Favourites", "Tags", "Network", "Devices"):
  - box 36 high: padding 14 top, 6 bottom, 10 sides
  - 12/500, colour t3
  - optional trailing 20×20 icon button (`…` or `+`)
- **Row:** height 32, padding 0 10, gap 10, radius 8.
  - icon 18 in t3, label 14/400 in t2
  - trailing `end` slot pushed right: 12 px t3 text and/or an 8 px status dot
  - hover: fill `hover`, label t1
  - selected: fill `fill2` (Light: black @ .065), label t1 500, icon accent
- **Tag dots:** a row with padding 4 10 6 and gap 10. Dots are 14×14 circles with an inset 1 px black @ .2 ring.
- **Device block:** padding 7 10 9, gap 6, radius 8.
  - top line: icon 18, name 14 t2 (flex), optional eject 22×22 / icon 14
  - bar: 4 high, radius 2, track (white @ .09 in Dark, black @ .08 in Light), fill accent (or warning when above 85% used)
  - caption: 12/16 t3, tabular
- Order: brand, Favourites (8 rows), Tags, Network, Devices. The sidebar scrolls if it overflows. The brand row stays pinned.

## Panel (content)
- Margin 8 top/right/bottom, 0 left, so it floats inside the window base.
- Radius 12, border 1 px `line`, background `panel`, clips its children.
- Vertical stack: toolbar 56, tabs bar 40 (always), content (flex), footer 38.

### Toolbar (height 56, padding 0 12 0 14, gap 10, align center)
Left to right:
1. **Nav group:**
   - 98×34 track: fill `fill`, radius 10, padding 2, gap 2
   - three 30×30 icon buttons, radius 8: Back (chevron-left 18), Forward (chevron-right 18; t4 when there is no history), Up (arrow-up 16)
2. **Title block:** margin-left 6, baseline-aligned, gap 10.
   - title 20/26/600 −1.5%
   - count 13 t3 tabular
3. **Spacer** (flex).
4. **Search:** 232×32, radius 10, fill `fill`, padding 0 6 0 10, gap 8.
   - search icon 16 t3
   - placeholder "Search ‹Folder›" 13 t3
   - keycap "/" on the right
5. **View switch:**
   - 130×34 track like the nav group
   - four 30×30 buttons with 16 icons: list, columns, grid, dual
   - selected: fill `fill3`, shadow control_on, icon t1
6. **Sort button:** height 32, padding 0 12, gap 7, radius 9, 13/500.
   - icon sort 16, label = the current key, chevron-down 14
   - ghost style (no fill) at rest; fill2 while its dropdown is open
7. **Settings:** 30×30 icon button, sliders 16.

### Tabs bar (height 40, padding 0 12, gap 4; rules top and bottom: 1 px `line`)
- **Tab:** height 30, padding 0 8 0 10, gap 8, radius 8, 13/400 t2.
  - leading icon 15 t3 (the place's sidebar icon)
  - active tab: fill `fill2` (Light: opaque white chip with the control_on shadow), label t1 500, plus a close button
  - close button: 18×18, radius 5, × at 12 in t3
- **New-tab button:** 26×26, plus 14.
- The tab bar is **always visible**, even with a single tab, so `+` is always one click away. With one tab, the tab has no × (closing it would mean closing the window: use Ctrl+W). The behaviour is in `10-user-journeys.md`.

### Content (Grid view)
- Padding 6 16 16. Vertical stack, gap 6. Scrolls vertically. Use `.virtualize()` by row of tiles.
- **Group header:** height 28, gap 8.
  - label 13/600 t2, −0.5%
  - count 13/500 t4
  - a 1 px `line` rule fills the rest
  - 14 px extra margin above every header except the first
- **Tile grid:** flex-wrap, gap 6. Tiles are 124 wide and **131 high**. The column count is whatever fits.
- **Tile:** padding 12 8 10, centred column, gap 8, radius 12.
  - art box: 68 high, centred
    - icon-theme icon 64×64, drop shadow
    - or a thumbnail 92×62 (radius 7, ring and shadow)
  - name: 13/17/500 t1, one line, ellipsis at the end, max width 108
  - meta: 11.5/14 t3, pulled up 6 (margin-top −6)
  - hover: fill `hover`
  - selected: fill `accentSoft` + inset 1 px `accentLine` + check badge
  - check badge: 18×18 circle, accent, check 11 px in `ink` at stroke 3.2, at top 7 right 7
  - cut: opacity .45
  - drop target: inset 1.5 px accent + fill accentSoft
- **Video thumbnail extras:**
  - duration chip bottom-right: 9.5/600 white on black @ .55, radius 4, padding 0 4, inset 4
  - play mark: 24 circle, black @ .45 with blur 6, play triangle 12 white, centred
- **Thumbnail size** scales with the footer slider. The reference is at 45% (tile 124). Range: 96 to 220 wide, aspect kept.

### Footer (height 38, padding 0 10 0 8, gap 10, top rule 1 px `line`, 12.5 t2)
- **Breadcrumb:**
  - crumbs are height 26, padding 0 7, gap 6, radius 7; icon 15 t3, label t2
  - separators: chevron-right 14 in t4
  - the last crumb is current: fill `fill`, label t1 500, icon accent
- Spacer.
- **Selection summary** (tabular): "1 of 16 selected · 4.1 MB", or "16 items" with nothing selected.
- A 1×14 divider in `line2`.
- **Free space:** 12.5 t3 tabular.
- **Icon-size slider** (Grid view only):
  - small grid icon 14 t3, then the track, then a large grid icon 20 t3
  - track 72×4, radius 2, `fill2`
  - filled part t3
  - knob 14 white circle with shadow `0 1 3 rgba(0,0,0,.5)`

## Menus (context menu, dropdowns, selects)
- **Container:**
  - 260 wide for context menus (240 for the sort dropdown, 232 for submenus, 200 for selects)
  - padding 5, radius 12
  - background `popover`; in Glass also backdrop blur sigma 14 and saturate 1.7
  - border 1 px `line2`, popover shadow
- **Item:** height 30, padding 0 8 0 10, gap 10, radius 7, 13.5 t1.
  - leading icon 16 t2, or a 16 px check slot (check icon in accent at stroke 2.4) for option menus
  - trailing content pushed right: keycaps, or 12 t3 text, or chevron-right 14
- **Highlighted item:** fill accent, text `ink`, icons and trailing at ink @ .72. Keycaps on it: background ink @ .12, border ink @ .14, text ink @ .7. Light mode: white @ .18 / .28 / white.
- **Destructive item:** text and icon `danger`.
- **Disabled item:** t4. Prefer hiding over disabling.
- **Separator:** 1 px `line2`, margin 5 8.
- **Section label:** 11.5/500 t3, padding 6 10 3.
- **Tag row** inside a menu: height 32, padding 0 10, gap 9, 14 px dots.
- **Keycap:**
  - min 20×20, padding 0 5, radius 5, 10.5/600 +3% t3
  - background white @ .07, border 1 px white @ .09 with a 2 px bottom border
  - key combos: separate caps, gap 3 ("Ctrl" "Shift" "C")
- **Placement:**
  - context menus open with their top-left at the pointer, flipped to stay inside the window, 8 px margin
  - submenus open at the parent's right edge −6, top aligned so their first item lines up with the parent item
  - dropdowns open 6 below their button, right-aligned to it

## List view (see `list-dark-*` references)
- Content padding 6 8 0. A list header row, then rows.
- **Grid columns:** name (flex, min 0) | kind 104 | size 76 (right-aligned) | modified 116. Column gap 14, row padding 0 12, radius 8.
- **Header:** height 30, 12/500 t3. The sorted column is t2 with a chevron-up/down 14.
- **Row:** height 32 (`flex: none`, never shrink), 13.5 t1.
  - name cell: icon-theme icon 20 + name + optional tag dots 7 px
  - kind cell: t3
  - size cell: t2, tabular, right-aligned; "—" in t4 when unknown
  - modified cell: t3, tabular
- **Row states:**
  - selected: accentSoft + inset 1 px accentLine
  - hover: `hover`
  - cut: opacity .45
- **Dates:**
  - "Today, 04:49" and "Yesterday, 18:02" for recent ones
  - "Nov 11, 14:13" within this year
  - "Sep 9, 2025" in older years
  - 24-hour clock by default
- **Inspector** (right, 284 wide, left rule `line`, padding 22 20, gap 16):
  - icon 104
  - name 17/600, meta 12.5 t3
  - buttons: small, height 26, padding 0 9, radius 7, 12
  - tag chips: height 24, radius 12, padding 0 9, fill `fill`
  - facts: rows with a top rule, 13 px; label column 72 wide in t3

## Settings sheet (see `settings-dark-*`)
- **Scrim:** black @ .5 over the window.
- **Sheet:**
  - 940×820, centred, radius 16
  - background `popover`; in Glass also blur
  - border `line2`, sheet shadow
- **Nav** (220 wide, padding 14 10, background white @ .02 in Dark, right rule `line`):
  - search field 32 high, margin-bottom 10
  - then sidebar-style rows
- **Header:** padding 20 20 4 28.
  - title 22/28/600
  - subtitle 13 t3
  - "Esc" keycap + close icon button on the right
- **Body:** padding 0 24 24, scrolls.
  - group title: 13/600 t2, padding 18 4 8
  - card: radius 12, fill white @ .03, border `line`
- **Setting row:** min-height 52, padding 9 14 9 16, gap 16, a top rule `line` between rows.
  - label 14/500 t1
  - optional description 12.5/17 t3
  - control right-aligned
- **Controls:**
  - switch: 34×20, knob 16 white, on = accent
  - segmented: track 2 px padding, segments 26 high, 12.5/500, radius 7
  - select: 30 high, 13/500, fill2, chevron 14
  - stepper: segmented with − value +
- **Dependent rows** (e.g. "Hidden files last" needs "Show hidden files"): whole row at opacity .45, and the description says why.
