# Tokens

Machine-readable: `tokens/tokens.json`. In Rust for Rune: `starter/theme.rs`.

Units are logical px (Rune points). Colours are sRGB. "X @ a" means colour X at alpha a, composited
source-over in **sRGB-encoded space**. That is Rune's default (`srgb_blend` on) and what CSS does, so the reference PNGs match.

## Surfaces and colour

### Dark mode

| Surface | Glass | Solid |
|---|---|---|
| base (window + sidebar) | rgba(18,20,30, 0.52) + backdrop blur 44px, saturate 1.7 | `#0F1013` |
| panel (content) | rgba(13,15,22, 0.8) + blur 44px, saturate 1.3 | `#17181C` |
| popover (menus, toasts, sheets) | rgba(32,35,48, 0.84) + blur 28px, saturate 1.7 | `#212328` |

| Token | Value | Used for |
|---|---|---|
| t1 · text primary | `#EDEFF5` | names, titles, menu items |
| t2 · text secondary | t1 @ 0.68 | sidebar rows, tab labels, footer text |
| t3 · text tertiary | t1 @ 0.52 | captions, meta, section labels, idle icons |
| t4 · text disabled | t1 @ 0.34 | disabled items, "—" placeholders, group counts |
| hover | white @ 0.05 | hovered rows and tiles |
| fill | white @ 0.06 | search field, segmented track, keycap-like chips |
| fill2 | white @ 0.1 | secondary button, selected sidebar row (dark), active tab, select |
| fill3 | white @ 0.15 | the selected segment / view button |
| line | white @ 0.07 | toolbar and footer dividers, list header rule, card rows |
| line2 | white @ 0.11 | menu border, menu separators, field ring |
| window border | white @ 0.1 | 1px window outline |
| accent | from theme (default `#7C9CFF`) | selection ring, menu highlight, focus, progress, check badge |
| accent soft | accent @ 0.2 | selection fill, drop target fill |
| accent line | accent @ 0.6 | 1px selection ring |
| ink | `#0B0D14` (or whichever of #0B0D14 / #FFFFFF contrasts more with the accent) | text and icons drawn on the accent |
| danger · warning · success | `#FF7A7A` · `#F2C55C` · `#62D6A0` | destructive items, capacity above 85%, connected dot / success toast |
| tag defaults | `#FF7A7A` `#FFA35C` `#F2C55C` `#62D6A0` `#5CCFD9` `#7C9CFF` `#B794FF` | only when the theme lacks that colour |

**Shadows** (x y blur colour alpha):

- window: 0 40 120 #000000 0.55, 0 8 24 #000000 0.35
- popover: 0 18 50 #000000 0.50, 0 2 8 #000000 0.30
- sheet: 0 40 120 #000000 0.60
- control_on: 0 1 2 #000000 0.30


### Light mode

| Surface | Glass | Solid |
|---|---|---|
| base (window + sidebar) | rgba(244,245,249, 0.62) + backdrop blur 44px, saturate 1.8 | `#ECEDF1` |
| panel (content) | rgba(255,255,255, 0.84) + blur 44px, saturate 1.4 | `#FFFFFF` |
| popover (menus, toasts, sheets) | rgba(255,255,255, 0.86) + blur 28px, saturate 1.8 | `#FFFFFF` |

| Token | Value | Used for |
|---|---|---|
| t1 · text primary | `#17191F` | names, titles, menu items |
| t2 · text secondary | t1 @ 0.7 | sidebar rows, tab labels, footer text |
| t3 · text tertiary | t1 @ 0.56 | captions, meta, section labels, idle icons |
| t4 · text disabled | t1 @ 0.38 | disabled items, "—" placeholders, group counts |
| hover | black @ 0.04 | hovered rows and tiles |
| fill | black @ 0.05 | search field, segmented track, keycap-like chips |
| fill2 | black @ 0.075 | secondary button, selected sidebar row (dark), active tab, select |
| fill3 | #FFFFFF (opaque) | the selected segment / view button |
| line | black @ 0.07 | toolbar and footer dividers, list header rule, card rows |
| line2 | black @ 0.1 | menu border, menu separators, field ring |
| window border | black @ 0.08 | 1px window outline |
| accent | from theme (default `#2F6BFF`) | selection ring, menu highlight, focus, progress, check badge |
| accent soft | accent @ 0.13 | selection fill, drop target fill |
| accent line | accent @ 0.55 | 1px selection ring |
| ink | `#FFFFFF` (or whichever of #0B0D14 / #FFFFFF contrasts more with the accent) | text and icons drawn on the accent |
| danger · warning · success | `#D63A4F` · `#C98A12` · `#2E9A5B` | destructive items, capacity above 85%, connected dot / success toast |
| tag defaults | `#D63A4F` `#D9652B` `#C98A12` `#2E9A5B` `#13929A` `#2F6BFF` `#8E52E0` | only when the theme lacks that colour |

**Shadows** (x y blur colour alpha):

- window: 0 40 100 #3C285A 0.22, 0 8 24 #3C285A 0.12
- popover: 0 18 50 #281E46 0.18, 0 2 8 #281E46 0.10, 0 0 0 0.5 #000000 0.06 (ring)
- sheet: 0 40 120 #281E46 0.22
- control_on: 0 1 2 #000000 0.12, 0 0 0 0.5 #000000 0.06 (ring)


### Light-mode specifics (differences from dark beyond the values above)
- Selected segment, view button and active tab: an opaque white chip with shadow `0 1 2 rgba(0,0,0,.12)` plus a 0.5 px ring `rgba(0,0,0,.06)`.
- Selected sidebar row: black @ 0.065.
- Keycaps: background black @ 0.04, border black @ 0.10 (2 px bottom border).
- Icons from the icon theme: drop shadow `0 2 4 rgba(30,40,80,.14)` (dark mode: `0 3 6 rgba(0,0,0,.28)`).
- Thumbnails: ring black @ 0.08 plus shadow `0 4 12 rgba(30,40,80,.18)` (dark mode: ring white @ 0.12 plus `0 6 14 rgba(0,0,0,.35)`).

## Type

Inter (Rune embeds Inter 4.1), with OpenType features `cv11` and `ss03` on. JetBrains Mono for paths and code only.
Every number (sizes, dates, counts, percentages) uses `tnum` (tabular figures).

| Role | Size / line | Weight | Tracking | Used for |
|---|---|---|---|---|
| display | 28 / 34 | 600 | −1.5% | big page titles (design docs) |
| title | 20 / 26 | 600 | −1.5% | toolbar folder title |
| title (sheet) | 22 / 28 | 600 | −1.5% | settings page title |
| heading | 15 / 20 | 600 | 0 | brand word "Rove", card titles |
| group header | 13 / 28 box | 600 | −0.5% | "Folders", "Files" |
| body | 14 / 20 | 400 | 0 | sidebar rows, settings labels (500) |
| list row | 13.5 / 20 | 400 | 0 | list view cells |
| menu item | 13.5 / 20 | 400 | 0 | context menu, dropdowns |
| label | 13 / 18 | 500 | 0 | buttons, tabs (13/400, active 500), search placeholder |
| tile name | 13 / 17 | 500 | 0 | grid tile name |
| caption | 12 / 16 | 400 | 0 | captions, sidebar section labels (500), list header (500) |
| tile meta | 11.5 / 14 | 400 | 0 | "48 items" under tile names |
| menu section label | 11.5 / 16 | 500 | 0 | "Sort by", "Open shore-at-dusk.jpg with" |
| keycap | 10.5 / 1 | 600 | +3% | Ctrl, Enter, Del |
| mono | 12.5 / 18 | 400 | 0 | paths, code |

## Space, radius, icon sizes
- **Spacing:** a 4 px grid: 4, 8, 12, 16, 20, 24, 32, 40, 48. Odd values that appear (6, 10, 14) are deliberate optical tweaks, listed in `04-layout.md`.
- **Radius:**
  - keycap 5
  - thumbnail 7
  - menu item 7
  - segment 7
  - row 8
  - tab 8
  - icon button 8
  - control (button, field, select) 9
  - segmented track 9
  - search 10
  - nav group 10
  - card 12
  - tile 12
  - menu 12
  - panel 12
  - toast 12
  - window 16
  - sheet 16
  - switch and chips: pill
- **Icon sizes** (stroke 1.75 at every size):
  - 20: inspector actions, large empty states
  - 18: sidebar rows, toolbar back/forward
  - 16: menus, view switch, search, sort button, settings button, Up arrow
  - 15: tabs, breadcrumb crumbs
  - 14: chevrons inside menus, eject, small "+" buttons, steppers
