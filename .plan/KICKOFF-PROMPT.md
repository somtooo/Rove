# Paste this into Claude Code on your machine

Historical scaffold request, completed. Do not use this as the current task:
`docs/08-milestones.md` now defines the vertical-slice sequence.

> I'm building **Rove**, a redesign of the Flea file manager (github.com/thisisgm/flea), as a new
> UI on the **Rune** GPU UI framework (github.com/anomalyco/rune), keeping Flea's backend.
> The complete design handoff is in `./rove-handoff/`. Read `rove-handoff/README.md`, then
> `SHAPE.md`, then `docs/08-milestones.md`.
>
> Do **Milestone 0 and Milestone 1 only**: scaffold `crates/rove-ui`, port `starter/theme.rs`,
> and build the **static, non-functional grid view** from mock data, in four scenes
> (dark-glass, dark-solid, latte-glass, latte-solid).
>
> Colours, sizes and type must match exactly. Get values from `tokens/tokens.json`,
> `docs/04-layout.md` and `reference/redlines.json`. When anything is ambiguous, open
> `reference/html/grid-dark-solid.html` and read the CSS.
>
> Verify headlessly with Rune's `--snap` / `--tree` as described in `docs/09-verification.md`.
> Iterate until geometry is within ±1 px and the colour spot checks are exact. Then show me the
> four snaps side by side with the references. Don't start Milestone 2 until I approve.
