# Theming: the Omarchy theme drives accents, never the look

The owner's principle: *being on Omarchy doesn't mean every app must be painted in the OS theme. It makes everything look the same, and contrast collapses.*
Rove reads the theme for **three things only**.

## The source
`~/.local/state/omarchy/current/theme/colors.toml`. `current` is a symlink that flips when the user changes theme.
Flea's `src/theme/watch.rs` already watches it, so reuse that. Values may be quoted with `"` or `'`.

## What is read

| colors.toml key | Rove token | Where it shows |
|---|---|---|
| `mode` (`light` / `dark`, missing = dark) | picks Rove's **Light** or **Dark** token set | everything, via Rove's own neutral values |
| `accent` | `accent`, `accentSoft`, `accentLine`, `ink` | selected tile/row fill and ring, check badge, menu highlight, focus ring, current tab chip text, current crumb icon, selected sidebar row icon, capacity and progress bars, switches on, primary button, text caret, drop targets |
| `red` `orange` `yellow` `green` `cyan` `blue` `magenta` | `tags[0..7]` | tag dots in sidebar, context menu, inspector chips, list rows |

## What is never read
`background`, `foreground`, `selection`, `muted`, every `*_background` and `*_foreground`, the `bright_*` colours, and any app-specific files in the theme folder.
Surfaces, text, lines and status colours are always Rove's own (`02-tokens.md`).

## Resolution rules
1. **Mode:** `mode == "light"` gives Light, anything else gives Dark.
2. **Accent:** the theme's `accent`, or Rove's default (Dark `#7C9CFF`, Light `#2F6BFF`) if missing or unparsable.
3. **Accent contrast fix:** the accent must reach **3:1** against the Solid panel colour (Dark `#17181C`, Light `#FFFFFF`). If it doesn't, step OKLab L by 0.01 (lighter in Dark, darker in Light), keeping hue and chroma, until it does. Respect monochrome themes (vantablack, white). Do not swap in a hue.
4. **Ink:** text and icons drawn *on* the accent use `#0B0D14` or `#FFFFFF`, whichever contrasts more with the resolved accent.
5. **Accent derivatives:**
   - accentSoft = accent @ 0.20 (Dark) / 0.13 (Light)
   - accentLine = accent @ 0.60 (Dark) / 0.55 (Light)
   - focus ring = 2 px accent, offset 2 px
6. **Tags:** use the seven theme colours as-is. They are identity colours and always appear with a name (tooltip or label). If a key is missing, use Rove's default for that slot.
7. **Switching themes:** animate colours over Rune `motion::CALM` (0.4 s, Oklab). Never re-layout.

## Every Omarchy theme, resolved (2026-11-13, branch `quattro`)
Contrast is the accent against the Solid panel. All pass 3:1 as shipped, so no adjustments are needed today. The rule is there for future themes.

| Theme | Mode | Theme accent | Rove accent | Contrast | Ink | Tags (R O Y G C B M) |
|---|---|---|---|---|---|---|
| catppuccin-latte | light | `#1E66F5` | `#1E66F5` | 4.91 | `#FFFFFF` | `#D20F39` `#D84E2B` `#DF8E1D` `#40A02B` `#179299` `#1E66F5` `#EA76CB` |
| catppuccin | dark | `#89B4FA` | `#89B4FA` | 8.42 | `#0B0D14` | `#F38BA8` `#F6B6AB` `#F9E2AF` `#A6E3A1` `#94E2D5` `#89B4FA` `#F5C2E7` |
| ethereal | dark | `#7D82D9` | `#7D82D9` | 5.12 | `#0B0D14` | `#ED5B5A` `#EB8B54` `#E9BB4F` `#92A593` `#A3BFD1` `#7D82D9` `#C89DC1` |
| everforest | dark | `#7FBBB3` | `#7FBBB3` | 8.16 | `#0B0D14` | `#E67E80` `#E09D7F` `#DBBC7F` `#A7C080` `#83C092` `#7FBBB3` `#D699B6` |
| flexoki-light | light | `#205EA6` | `#205EA6` | 6.54 | `#FFFFFF` | `#D14D41` `#D0772B` `#D0A215` `#879A39` `#3AA99F` `#205EA6` `#CE5D97` |
| gruvbox | dark | `#7DAEA3` | `#7DAEA3` | 7.14 | `#0B0D14` | `#EA6962` `#E1875C` `#D8A657` `#A9B665` `#89B482` `#7DAEA3` `#D3869B` |
| hackerman | dark | `#82FB9C` | `#82FB9C` | 13.71 | `#0B0D14` | `#50F872` `#50F7A3` `#50F7D4` `#4FE88F` `#7CF8F7` `#829DD4` `#86A7DF` |
| jade-horse | dark | `#8BCBB1` | `#8BCBB1` | 9.52 | `#0B0D14` | `#D88D91` `#D2AB8D` `#C5CC91` `#8BCBB1` `#86C7C5` `#96B7D0` `#B7A1C8` |
| kanagawa | dark | `#DCD7BA` | `#DCD7BA` | 12.22 | `#0B0D14` | `#C34043` `#C17158` `#C0A36E` `#76946A` `#6A9589` `#7E9CD8` `#957FB8` |
| last-horizon | dark | `#B59790` | `#B59790` | 6.58 | `#0B0D14` | `#C38B7B` `#FFA35C` `#6B5E73` `#87A9B0` `#A5A0B6` `#B59790` `#C4D8E2` |
| lumon | dark | `#8BC9EB` | `#8BC9EB` | 9.84 | `#0B0D14` | `#4D86B0` `#8BC9EB` `#6FA4C9` `#5E95BC` `#B4E4F6` `#6FB8E3` `#8BC9EB` |
| lupine | light | `#3264EB` | `#3264EB` | 5.05 | `#FFFFFF` | `#C900C4` `#026FDE` `#026FDE` `#4A2FD0` `#0C67DE` `#3264EB` `#8A4AD7` |
| matte-black | dark | `#E68E0D` | `#E68E0D` | 6.96 | `#0B0D14` | `#D35F5F` `#C63D3D` `#B91C1C` `#FFC107` `#BEBEBE` `#E68E0D` `#D35F5F` |
| miasma | dark | `#78824B` | `#78824B` | 4.3 | `#0B0D14` | `#685742` `#8D6242` `#B36D43` `#5F875F` `#C9A554` `#78824B` `#BB7744` |
| nord | dark | `#81A1C1` | `#81A1C1` | 6.59 | `#0B0D14` | `#BF616A` `#D5967A` `#EBCB8B` `#A3BE8C` `#88C0D0` `#81A1C1` `#B48EAD` |
| retro-82 | dark | `#FAA968` | `#FAA968` | 9.24 | `#0B0D14` | `#F85525` `#FAA968` `#E97B3C` `#028391` `#8CBFB8` `#3F8F8A` `#3F8F8A` |
| ristretto | dark | `#F38D70` | `#F38D70` | 7.48 | `#0B0D14` | `#FD6883` `#FB9A77` `#F9CC6C` `#ADDA78` `#85DACC` `#F38D70` `#A8A9EB` |
| rose-pine | light | `#56949F` | `#56949F` | 3.42 | `#0B0D14` | `#B4637A` `#CF8057` `#EA9D34` `#286983` `#D7827E` `#56949F` `#907AA9` |
| solitude | dark | `#798186` | `#798186` | 4.47 | `#0B0D14` | `#565D60` `#FFA35C` `#D9DBDC` `#9FA5A9` `#707070` `#798186` `#AEAEAE` |
| starship | dark | `#FF9F0A` | `#FF9F0A` | 8.63 | `#0B0D14` | `#FF5F4A` `#FF6A3D` `#FFC27A` `#FF9F0A` `#A6DCEF` `#8FB8DE` `#FF8FA3` |
| tokyo-night | dark | `#7AA2F7` | `#7AA2F7` | 7.04 | `#0B0D14` | `#F7768E` `#EB927B` `#E0AF68` `#9ECE6A` `#449DAB` `#7AA2F7` `#AD8EE6` |
| vantablack | dark | `#8D8D8D` | `#8D8D8D` | 5.34 | `#0B0D14` | `#A4A4A4` `#B9B9B9` `#CECECE` `#B6B6B6` `#B0B0B0` `#8D8D8D` `#9B9B9B` |
| white | light | `#6E6E6E` | `#6E6E6E` | 5.1 | `#FFFFFF` | `#2A2A2A` `#D9652B` `#4A4A4A` `#3A3A3A` `#3E3E3E` `#1A1A1A` `#2E2E2E` |

Notes:
- **kanagawa**'s accent equals its foreground (a warm off-white). Rove uses it as-is: selection becomes cream with dark ink. That's the theme's choice.
- **hackerman**'s tags are all greens. Tags stay distinguishable by name.
- **last-horizon**, **solitude** and **white** have no `magenta`, so slot 7 uses Rove's default purple.

## Worked example: Catppuccin Latte
`mode = "light"`, `accent = "#1e66f5"`.
- Rove switches to Light material. Its neutrals stay Rove's (panel `#FFFFFF`), **not** Latte's `#eff1f5`.
- accent `#1E66F5` (4.91:1 on white) with ink `#FFFFFF`.
- Tags: `#D20F39 #D84E2B #DF8E1D #40A02B #179299 #1E66F5 #EA76CB`.
- See `reference/screens/grid-latte-glass@2x.png` and `grid-latte-solid@2x.png`.
