# Icons

## Interface icons: Rove's own set (`icons/`)
- **Files:** 61 SVGs in `icons/svg/`, plus `icons/icons.json` (name → path data).
- **Grid:** 24×24, `fill="none"`, `stroke="currentColor"`, stroke-width **1.75**, round caps and joins.
- **Sizes:** draw at 14, 16, 18, 20 or 24 and keep the 1.75 stroke (don't scale it up with the size).
- **Colour** comes from the token in context: t3 idle in the sidebar, t2 in menus, accent when selected, ink on accent, danger when destructive.
- **Adding icons:** draw on the same grid with the same stroke. The geometry follows Lucide (ISC); the set is redrawn as needed. Never use square caps.

| Name | Used for |
|---|---|
| home, download, doc, image, music, video, code, trash | sidebar places, tab icons, crumbs |
| cloud, server, drive, usb, phone, eject, lock | network and devices |
| chevL, chevR, up, chevD, chevU | navigation, submenus, sort direction |
| search, list, columns, grid, dual, sort, sliders, sidebar, panel | toolbar |
| open, copy, cut, pencil, archive, eye, info, link, shelf, share, tag, terminal, star, undo, refresh | menus |
| keyboard, plug, monitor, menu, type | settings navigation |
| plus, minus, x, check, more, git, clock, disk, zip, globe, pin, bolt | general |

`icons/preview.png` is a contact sheet of the whole set.

## File and folder icons: from the user's icon theme
- Implement the freedesktop **Icon Theme Specification**.
  - The active theme comes from `gsettings get org.gnome.desktop.interface icon-theme`, or GTK `settings.ini` / `kdeglobals`.
  - Follow `Inherits=`, fall back to `hicolor`.
  - Prefer `scalable`, else the nearest size at or above the target × scale.
- Map MIME types to icon names with shared-mime-info (`generic-icon` and `icon` from the MIME database; e.g. `application/x-shellscript` → `application-x-shellscript`, then `text-x-script`).
- **Special folders** use their XDG icon names: `folder-documents`, `folder-download`, `folder-pictures` (WhiteSur names it `folder-images`), `folder-music`, `folder-videos`, `folder-projects`, plus `user-home`, `user-trash` and `user-desktop`.
- **States:** `folder-locked` (encrypted, not unlocked), `folder-cloud` (network mount), `folder-code` (a git repo or project; optional heuristic).
- **Sizes:**
  - grid: 64 (art box 68)
  - list: 20
  - inspector: 104
  - sidebar never uses icon-theme icons; it always uses interface icons
- **Drop shadow** on theme icons: Dark `0 3 6 rgba(0,0,0,.28)`, Light `0 2 4 rgba(30,40,80,.14)`. List-size icons have no shadow.
- WhiteSur is GPL-3.0. The PNGs in `reference/html/assets/` are reference only. Never bundle a theme. If no theme is installed, fall back to `hicolor` and then to Rove's interface `doc` / `folder`-style icon in t3.
- **Brand marks:** don't draw other companies' logos (Adobe, Apple…) in Rove's own assets. Themes that include them are the user's choice.

## Thumbnails (photos, video, PDF first page)
- Produced by the backend `thumb` command for visible items only.
- Grid: 92×62 (aspect-fit, then cover-cropped), radius 7. Ring: white @ .12 (Light: black @ .08). Shadow: thumbnail token.
- Video: duration chip and play mark (see `04-layout.md`).
- While loading, show the theme icon; cross-fade to the thumbnail over 150 ms.
